use crate::merkle_tree::field_based_mht::smt::{SmtPoseidonParameters, Coord, OperationLeaf};
use crate::{PoseidonParameters, PoseidonHash, FieldBasedHash, merkle_tree};
use crate::merkle_tree::field_based_mht::smt::ActionLeaf::Insert;

use algebra::{PrimeField, MulShort};
use algebra::{ToBytes, to_bytes};

use rocksdb::DB;

use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::fs;
use crate::merkle_tree::field_based_mht::smt::error::Error;

#[derive(Debug)]
pub struct BigMerkleTree<F: PrimeField, T: SmtPoseidonParameters<Fr=F>, P: PoseidonParameters<Fr=F>> {

    // the number of leaves
    width: usize,
    // the height of the Merkle tree
    height: usize,
    // path to the db
    path_db: String,
    // stores the leaves
    database: DB,
    // path to the cache
    path_cache: String,
    // stores the cached nodes
    db_cache: DB,
    // stores the nodes of the path
    cache_path: HashMap<Coord, F>,
    // indicates which nodes are present the Merkle tree
    present_node: HashSet<Coord>,
    // root of the Merkle tree
    root: F,

    _field: PhantomData<F>,
    _parameters: PhantomData<T>,
    _poseidon_parameters: PhantomData<P>,
}

impl<F: PrimeField, T: SmtPoseidonParameters<Fr=F>, P: PoseidonParameters<Fr=F>> Drop for BigMerkleTree<F, T, P> {
    fn drop(&mut self) {
        // Deletes the folder containing the db
        match fs::remove_dir_all(self.path_db.clone()) {
            Ok(_) => (),
            Err(e) => {
                println!("Error deleting the folder containing the db. {}", e);
            }
        };
        // Deletes the folder containing the cache
        match fs::remove_dir_all(self.path_cache.clone()) {
            Ok(_) => (),
            Err(e) => {
                println!("Error deleting the folder containing the db. {}", e);
            }
        };
    }
}

// Assumption: MERKLE_ARITY == 2
/*************************************************************************************/
//impl<F: PrimeField + MulShort, T: SmtPoseidonParameters<Fr=F>, P: PoseidonParameters<Fr=F>> BigMerkleTree<F, T, P> {
    /*************************************************************************************/
impl<F: PrimeField, T: SmtPoseidonParameters<Fr=F>, P: PoseidonParameters<Fr=F>> BigMerkleTree<F, T, P> {
    pub fn new(width: usize, path_db: String, path_cache: String) -> Result<Self, Error> {
        let height = width as f64;
        let height = height.log(T::MERKLE_ARITY as f64) as usize;
        let path_db = path_db;
        let database = DB::open_default(path_db.clone()).unwrap();
        let path_cache = path_cache;
        let db_cache = DB::open_default(path_cache.clone()).unwrap();
        let cache_path = HashMap::new();
        let present_node = HashSet::new();
        let root = T::EMPTY_HASH_CST[height];
        Ok(BigMerkleTree {
            width,
            height,
            path_db,
            database,
            path_cache,
            db_cache,
            cache_path,
            present_node,
            root,
            _field: PhantomData,
            _parameters: PhantomData,
            _poseidon_parameters: PhantomData,
        })
    }

    pub fn insert_to_cache(&self, coord: Coord, data:F) {
        let elem = to_bytes!(data).unwrap();
        let index = bincode::serialize(&coord).unwrap();
        self.database.put(index, elem).unwrap();
    }

    pub fn contains_key_in_cache(&self, coord:Coord) -> bool {
        let coordinates = bincode::serialize(&coord).unwrap();
        match self.database.get(coordinates) {
            Ok(Some(_value)) => {
                return true;
            },
            Ok(None) => {
                return false;
            },
            Err(e) => {
                println!("operational problem encountered: {}", e);
                return false;
            },
        }
    }

    pub fn get_from_cache(&self, coord:Coord) -> Option<F> {
        let coordinates = bincode::serialize(&coord).unwrap();
        match self.database.get(coordinates) {
            Ok(Some(value)) => {
                let retrieved_elem = F::read(value.as_slice()).unwrap();
                return Some(retrieved_elem);
            },
            Ok(None) => {
                return None;
            },
            Err(e) => {
                println!("operational problem encountered: {}", e);
                return None;
            },
        }
    }

    pub fn remove_from_cache(&self, coord: Coord) -> Option<F>{
        let coordinates = bincode::serialize(&coord).unwrap();
        match self.database.get(coordinates.clone()) {
            Ok(Some(value)) => {
                let retrieved_elem = F::read(value.as_slice()).unwrap();
                let res = self.database.delete(coordinates.clone());
                match res {
                    Ok(_) => {
                        return Some(retrieved_elem);
                    },
                    Err(e) => {
                        println!("Could not delete node from cache: {}", e);
                        return None;
                    }
                }
            },
            Ok(None) => {
                return None;
            },
            Err(e) => {
                println!("operational problem encountered: {}", e);
                return None;
            },
        }
    }

    pub fn insert_to_db(&self, idx: usize, data: F) {
        let elem = to_bytes!(data).unwrap();
        let index = bincode::serialize(&idx).unwrap();
        self.database.put(index, elem).unwrap();
    }

    pub fn get_from_db(&self, idx: usize) -> Option<F>{
        let index = bincode::serialize(&idx).unwrap();
        match self.database.get(index) {
            Ok(Some(value)) => {
                let retrieved_elem = F::read(value.as_slice()).unwrap();
                return Some(retrieved_elem);
            },
            Ok(None) => {
                return None;
            },
            Err(e) => {
                println!("operational problem encountered: {}", e);
                return None;
            },
        }
    }

    pub fn remove_from_db(&self, idx: usize) -> Option<F>{
        let index = bincode::serialize(&idx).unwrap();
        match self.database.get(index.clone()) {
            Ok(Some(value)) => {
                let retrieved_elem = F::read(value.as_slice()).unwrap();
                let res = self.database.delete(index.clone());
                match res {
                    Ok(_) => {
                        return Some(retrieved_elem);
                    },
                    Err(e) => {
                        println!("Could not delete leaf from db: {}", e);
                        return None;
                    }
                }
            },
            Ok(None) => {
                return None;
            },
            Err(e) => {
                println!("operational problem encountered: {}", e);
                return None;
            },
        }
    }


    // Inserts a leaf in the Merkle tree. Only used for single operation
    // Updates the Merkle tree on the path from the leaf to the root
    pub fn insert_leaf(&mut self, coord: Coord, leaf: F) {

        // check that the index of the leaf to be inserted is less than the width of the Merkle tree
        assert!(coord.idx < self.width, "Leaf index out of bound.");
        // check that the coordinates of the node corresponds to the leaf level
        assert_eq!(coord.height, 0, "Coord of the node does not correspond to leaf level");

        if self.present_node.contains(&coord) {
            let old_leaf = self.get_from_db(coord.idx);
            let old_hash;
            if let Some(i) = old_leaf {
                old_hash = i;
            } else {
                old_hash = T::EMPTY_HASH_CST[0];
            }
            if old_hash != leaf {
                self.insert_to_db(coord.idx, leaf);
                self.cache_path.clear();
                self.cache_path.insert(coord, leaf);
                BigMerkleTree::update_tree(self, coord);
            }
        } else {
            // mark as non empty leaf
            self.present_node.insert(coord);
            self.insert_to_db(coord.idx, leaf);
            self.cache_path.clear();
            self.cache_path.insert(coord, leaf);
            BigMerkleTree::update_tree(self, coord);
        }
    }

    // Removes a leaf in the Merkle tree. Only used for single operation
    // Updates the Merkle tree on the path from the leaf to the root
    pub fn remove_leaf(&mut self, coord: Coord) {

        // check that the index of the leaf to be inserted is less than the width of the Merkle tree
        assert!(coord.idx < self.width, "Leaf index out of bound.");
        // check that the coordinates of the node corresponds to the leaf level
        assert_eq!(coord.height, 0, "Coord of the node does not correspond to leaf level");

        // take that leaf from the non-empty set
        self.present_node.remove(&coord);
        self.cache_path.clear();
        self.cache_path.insert(coord, T::EMPTY_HASH_CST[0]);
        // removes the leaf from the db
        let res = self.remove_from_db(coord.idx);
        // if it was in the db, update the tree
        if res != None {
            BigMerkleTree::update_tree(self, coord);
        }
    }

    // Updates the tree visiting the parent nodes from the leaf to the root
    // Calculates the hash and caches it
    pub fn update_tree(&mut self, coord: Coord) {

        // Process the node of level 1 with the inserted/removed leaf
        // check whether the hash corresponds to the left or right child
        let mut idx = coord.idx;
        let left_child_coord: Coord;
        let right_child_coord: Coord;
        let left_child_idx: usize;
        let right_child_idx: usize;
        let left_child: Option<F>;
        let right_child: Option<F>;
        let left_hash: F;
        let right_hash: F;
        let mut height = 0;

        assert_eq!(T::MERKLE_ARITY,2, "Arity of the Merkle tree is not 2.");

        if idx % T::MERKLE_ARITY == 0 {
            left_child_idx = idx;
            left_child_coord = Coord { height, idx: left_child_idx };
            // get the left child from the cache_path
            let hash = self.cache_path.get(&left_child_coord);
            if let Some(i) = hash {
                left_hash = *i;
            } else {
                left_hash = T::EMPTY_HASH_CST[0];
            }
            right_child_idx = idx + 1;
            right_child_coord = Coord { height, idx: right_child_idx };
            right_child = self.get_from_db(right_child_idx);
            if let Some(i) = right_child {
                right_hash = i;
            } else {
                right_hash = T::EMPTY_HASH_CST[0];
            }
        } else {
            right_child_idx = idx;
            right_child_coord = Coord { height, idx: right_child_idx };
            // get the right child from the cache path
            let hash = self.cache_path.get(&right_child_coord);
            if let Some(i) = hash {
                right_hash = *i;
            } else {
                right_hash = T::EMPTY_HASH_CST[0];
            }
            left_child_idx = idx - 1;
            left_child_coord = Coord { height, idx: left_child_idx };
            left_child = self.get_from_db(left_child_idx);
            if let Some(i) = left_child {
                left_hash = i;
            } else {
                left_hash = T::EMPTY_HASH_CST[0];
            }
        }

        height += 1;
        idx = idx / T::MERKLE_ARITY;
        let parent_coord = Coord { height, idx };

        let mut node_hash: F;
        if (!self.present_node.contains(&left_child_coord)) & (!self.present_node.contains(&right_child_coord)) {
            node_hash = T::EMPTY_HASH_CST[height];

            // insert the parent node into the cache_path
            self.cache_path.insert(parent_coord, node_hash);

            // Both children are empty leaves
            // remove the parent node from the presence set
            self.present_node.remove(&parent_coord.clone());
            // remove node from cache
            BigMerkleTree::remove_node_from_cache(self, parent_coord.clone());
        } else {

            // compute the hash of the node with the hashes of the children
            node_hash = merkle_tree::field_based_mht::smt::big_merkle_tree::BigMerkleTree::<F, T, P>::poseidon_hash(left_hash, right_hash);
            // insert the parent node into the cache_path
            self.cache_path.insert(parent_coord, node_hash);
            // set the parent as present
            self.present_node.insert(parent_coord.clone());
            // Both children are not empty leaves
            if (self.present_node.contains(&left_child_coord)) & (self.present_node.contains(&right_child_coord)) {
                // cache the node
                BigMerkleTree::insert_node_in_cache(self, parent_coord.clone(), node_hash);
            }
        }

        // Process level >= 2
        while height != self.height {
            // go to parent node
            height += 1;
            let idx_child = idx;
            idx = idx / T::MERKLE_ARITY;
            let parent_coord = Coord { height, idx };

            // retrieve the left child
            let left_child_idx = parent_coord.idx * T::MERKLE_ARITY;
            let left_child_height = parent_coord.height - 1;
            let left_child_coord = Coord { height: left_child_height, idx: left_child_idx };
            let left_hash: F;
            if left_child_idx == idx_child {
                // It was cached in the previous iteration. Get it from the cache_path
                left_hash = *self.cache_path.get(&left_child_coord).unwrap();
            } else {
                left_hash = BigMerkleTree::node(self, left_child_coord);
            }

            // retrieve the right child
            let right_child_idx = left_child_idx + 1;
            let right_child_height = left_child_height;
            let right_child_coord = Coord { height: right_child_height, idx: right_child_idx };
            let right_hash: F;
            if right_child_idx == idx_child {
                // It was cached in the previous iteration. Get it from the cache_path
                right_hash = *self.cache_path.get(&right_child_coord).unwrap();
            } else {
                right_hash = BigMerkleTree::node(self, right_child_coord);
            }

            if (!self.present_node.contains(&left_child_coord)) & (!self.present_node.contains(&right_child_coord)) {
                node_hash = T::EMPTY_HASH_CST[height];
                // insert the parent node into the cache_path
                self.cache_path.insert(parent_coord, node_hash);
                // remove node from non_empty set
                self.present_node.remove(&parent_coord.clone());
            } else {
                // compute the hash of the parent node based on the hashes of the children
                node_hash = merkle_tree::field_based_mht::smt::big_merkle_tree::BigMerkleTree::<F, T, P>::poseidon_hash(left_hash, right_hash);
                // insert the parent node into the cache_path
                self.cache_path.insert(parent_coord, node_hash);
                // both children are present
                if self.present_node.contains(&left_child_coord) & self.present_node.contains(&right_child_coord) {
                    // set the parent node as non_empty
                    self.present_node.insert(parent_coord.clone());
                    // children not empty leaves, then cache the parent node
                    BigMerkleTree::insert_node_in_cache(self, parent_coord.clone(), node_hash);
                    // cache the children
                    BigMerkleTree::insert_node_in_cache(self, left_child_coord.clone(), left_hash);
                    BigMerkleTree::insert_node_in_cache(self, right_child_coord.clone(), right_hash);
                } else {
                    // either child not equal to empty leaves, include the parent node in a non-empty set
                    self.present_node.insert(parent_coord.clone());
                }
            }
            if (!self.present_node.contains(&left_child_coord)) | (!self.present_node.contains(&right_child_coord)) {
                if self.contains_key_in_cache(parent_coord) {
                    // remove subtree from cache
                    BigMerkleTree::remove_subtree_from_cache(self, parent_coord, 2);
                }
            }
        }
        self.root = node_hash;
    }

    /*******CHECK ******************/
    pub fn remove_subtree_from_cache(&mut self, coord: Coord, depth: usize) {

        if depth == 0 {
            return;
        }
        let left_child_idx = coord.idx * T::MERKLE_ARITY;
        let left_child_height = 0;
        let left_coord = Coord { height: left_child_height, idx: left_child_idx };

        let right_child_idx = left_child_idx + 1;
        let right_child_height = 0;
        let right_coord = Coord { height: right_child_height, idx: right_child_idx };

        if (!self.present_node.contains(&left_coord)) & (!self.present_node.contains(&right_coord)) {
            if self.contains_key_in_cache(coord) {
                BigMerkleTree::remove_node_from_cache(self, coord);
            }
            if !self.present_node.contains(&coord) {
                self.present_node.remove(&coord.clone());
            }
            return;
        }
        if (!self.present_node.contains(&left_coord)) | (!self.present_node.contains(&right_coord)) {
            if self.contains_key_in_cache(coord) {
                BigMerkleTree::remove_node_from_cache(self, coord);
            }
            if self.present_node.contains(&left_coord) {
                self.remove_subtree_from_cache(left_coord, depth -1);
            }
            if self.present_node.contains(&right_coord) {
                self.remove_subtree_from_cache(right_coord, depth -1);
            }
        }
        return;
    }


    // Returns the hash value associated to the node.
    // If the node is in the cache, it retrieves from it.
    // If not, recomputes it.
    // Only used for nodes of level >= 1 (not for leaves).
    pub fn node(&mut self, coord: Coord) -> F {

        assert_eq!(T::MERKLE_ARITY,2, "Arity of the Merkle tree is not 2.");

        //let coord = Coord{height, idx};
        // if the node is an empty node return the hash constant
        if !self.present_node.contains(&coord) {
            return T::EMPTY_HASH_CST[coord.height];
        }
        let res = self.get_from_cache(coord);

        // if not in the cache, recompute it
        if res == None {
            /* the node is not in the cache, compute it */
            let node_hash;
            if coord.height == 1 {
                /* get leaves to compute */
                let left_child_idx = coord.idx * T::MERKLE_ARITY;
                let left_child = self.get_from_db(left_child_idx);
                let left_hash: F;
                if let Some(i) = left_child {
                    left_hash = i;
                } else {
                    left_hash = T::EMPTY_HASH_CST[0];
                }

                let right_child_idx = left_child_idx + 1;
                let right_child = self.get_from_db(right_child_idx);
                let right_hash: F;
                if let Some(i) = right_child {
                    right_hash = i;
                } else {
                    right_hash = T::EMPTY_HASH_CST[0];
                }
                node_hash = merkle_tree::field_based_mht::smt::big_merkle_tree::BigMerkleTree::<F, T, P>::poseidon_hash(left_hash, right_hash);
            } else {
                let height_child = coord.height - 1;
                let left_child_idx = coord.idx * T::MERKLE_ARITY;
                let coord_left = Coord { height: height_child, idx: left_child_idx };
                let left_child_hash = BigMerkleTree::node(self, coord_left);

                let right_child_idx = left_child_idx + 1;
                let coord_right = Coord { height: height_child, idx: right_child_idx };
                let right_child_hash = BigMerkleTree::node(self, coord_right);

                node_hash = merkle_tree::field_based_mht::smt::big_merkle_tree::BigMerkleTree::<F, T, P>::poseidon_hash(left_child_hash, right_child_hash);
            }
            return node_hash;
        }

        res.unwrap()
    }

    pub fn insert_node_in_cache(&mut self, coord: Coord, hash: F) {
        self.insert_to_cache(coord, hash);
    }

    pub fn remove_node_from_cache(&mut self, coord: Coord) {
        self.remove_from_cache(coord);
    }

    pub fn poseidon_hash(x: F, y: F) -> F {
        let mut input = Vec::new();
        input.push(x);
        input.push(y);
        let hash = PoseidonHash::<F, P>::evaluate(&input[..]);
        hash.unwrap()
    }

    pub fn process_leaves_normal(&mut self, lidx: Vec<OperationLeaf<F>>) -> F {

        for k in 0..lidx.len() {
            let x = lidx[k];
            let coord = x.coord;
            let action = x.action;
            let hash = x.hash;

            if action == Insert {
                self.insert_leaf(coord, hash.unwrap());
            } else {
                self.remove_leaf(coord);
            }
        }
        self.root
    }

}


#[cfg(test)]
mod test {
    use crate::merkle_tree::field_based_mht::smt::{MNT4PoseidonHash, OperationLeaf, Coord, ActionLeaf};
    use crate::merkle_tree::field_based_mht::{FieldBasedMerkleTreeConfig, FieldBasedMerkleHashTree};
    use crate::merkle_tree::field_based_mht::smt::parameters::{MNT4753SmtPoseidonParameters, MNT6753SmtPoseidonParameters};
    use crate::MNT6PoseidonHash;
    use crate::crh::poseidon::parameters::{MNT6753PoseidonParameters, MNT4753PoseidonParameters};

    use algebra::fields::mnt6753::Fr as MNT6753Fr;
    use algebra::fields::mnt4753::Fr as MNT4753Fr;

    use std::str::FromStr;

    use rand_xorshift::XorShiftRng;
    use rand::SeedableRng;
    use crate::merkle_tree::field_based_mht::smt::big_merkle_tree::BigMerkleTree;

    pub type MNT4PoseidonSmt = BigMerkleTree< MNT4753Fr, MNT4753SmtPoseidonParameters, MNT4753PoseidonParameters>;

    struct MNT4753FieldBasedMerkleTreeParams;

    impl FieldBasedMerkleTreeConfig for MNT4753FieldBasedMerkleTreeParams {
        const HEIGHT: usize = 6;
        type H = MNT4PoseidonHash;
    }

    type MNT4753FieldBasedMerkleTree = FieldBasedMerkleHashTree<MNT4753FieldBasedMerkleTreeParams>;

    #[test]
    fn compare_merkle_trees_mnt4_1() {
        use algebra::{
            fields::mnt4753::Fr, Field,
        };

        let num_leaves = 32;
        let mut leaves_to_process: Vec<OperationLeaf<MNT4753Fr>> = Vec::new();

        leaves_to_process.push(OperationLeaf { coord: Coord { height: 0, idx: 0 }, action: ActionLeaf::Insert, hash: Some(MNT4753Fr::from_str("1").unwrap()) });
        leaves_to_process.push(OperationLeaf { coord: Coord { height: 0, idx: 9 }, action: ActionLeaf::Insert, hash: Some(MNT4753Fr::from_str("2").unwrap()) });
        leaves_to_process.push(OperationLeaf { coord: Coord { height: 0, idx: 16 }, action: ActionLeaf::Remove, hash: Some(MNT4753Fr::from_str("3").unwrap()) });
        leaves_to_process.push(OperationLeaf { coord: Coord { height: 0, idx: 29 }, action: ActionLeaf::Insert, hash: Some(MNT4753Fr::from_str("3").unwrap()) });
        leaves_to_process.push(OperationLeaf { coord: Coord { height: 0, idx: 16 }, action: ActionLeaf::Remove, hash: Some(MNT4753Fr::from_str("3").unwrap()) });

        let mut smt = MNT4PoseidonSmt::new(num_leaves, String::from("./db_leaves") , String::from("./db_cache")).unwrap();
        smt.process_leaves_normal(leaves_to_process);

        //=============================================

        let mut leaves = Vec::new();
        leaves.push(MNT4753Fr::from_str("1").unwrap());
        for _ in 1..9 {
            let f = Fr::zero();
            leaves.push(f);
        }
        leaves.push(MNT4753Fr::from_str("2").unwrap());
        for _ in 10..29 {
            let f = Fr::zero();
            leaves.push(f);
        }
        leaves.push(MNT4753Fr::from_str("3").unwrap());
        for _ in 30..32 {
            let f = Fr::zero();
            leaves.push(f);
        }
        let tree = MNT4753FieldBasedMerkleTree::new(&leaves).unwrap();

        assert_eq!(tree.root(), smt.root, "Outputs of the Merkle trees for MNT4 do not match.");
    }

    #[test]
    fn compare_merkle_trees_mnt4_2() {
        use algebra::{
            fields::mnt4753::Fr,
            UniformRand,
        };

        let mut rng = XorShiftRng::seed_from_u64(9174123u64);
        let num_leaves = 32;
        let mut leaves = Vec::new();
        for _ in 0..num_leaves {
            let f = Fr::rand(&mut rng);
            leaves.push(f);
        }
        let tree = MNT4753FieldBasedMerkleTree::new(&leaves).unwrap();

        let mut smt = MNT4PoseidonSmt::new(num_leaves,String::from("./db_leaves") , String::from("./db_cache")).unwrap();
        let mut rng = XorShiftRng::seed_from_u64(9174123u64);
        for i in 0..num_leaves {
            let f = Fr::rand(&mut rng);
            smt.insert_leaf(
                Coord{height:0, idx:i},
                f,
            );
        }

        assert_eq!(tree.root(), smt.root, "Outputs of the Merkle trees for MNT4 do not match.");
    }

    #[test]
    fn compare_merkle_trees_mnt4_3() {
        use algebra::{
            fields::mnt4753::Fr, Field,
        };

        let num_leaves = 32;
        let mut leaves = Vec::new();
        leaves.push(MNT4753Fr::from_str("1").unwrap());
        for _ in 1..9 {
            let f = Fr::zero();
            leaves.push(f);
        }
        leaves.push(MNT4753Fr::from_str("2").unwrap());
        for _ in 10..29 {
            let f = Fr::zero();
            leaves.push(f);
        }
        leaves.push(MNT4753Fr::from_str("3").unwrap());
        for _ in 30..32 {
            let f = Fr::zero();
            leaves.push(f);
        }
        let tree = MNT4753FieldBasedMerkleTree::new(&leaves).unwrap();

        let mut smt = MNT4PoseidonSmt::new(num_leaves, String::from("./db_leaves") , String::from("./db_cache")).unwrap();
        smt.insert_leaf(Coord{height:0, idx:0}, MNT4753Fr::from_str("1").unwrap());
        smt.insert_leaf(Coord{height:0, idx:9}, MNT4753Fr::from_str("2").unwrap());
        smt.insert_leaf(Coord{height:0, idx:16}, MNT4753Fr::from_str("10").unwrap());
        smt.insert_leaf(Coord{height:0, idx:29}, MNT4753Fr::from_str("3").unwrap());
        smt.remove_leaf(Coord{height:0, idx:16});

        assert_eq!(tree.root(), smt.root, "Outputs of the Merkle trees for MNT4 do not match.");
    }

    struct MNT6753FieldBasedMerkleTreeParams;

    impl FieldBasedMerkleTreeConfig for MNT6753FieldBasedMerkleTreeParams {
        const HEIGHT: usize = 6;
        type H = MNT6PoseidonHash;
    }

    type MNT6753FieldBasedMerkleTree = FieldBasedMerkleHashTree<MNT6753FieldBasedMerkleTreeParams>;

    pub type MNT6PoseidonSmt = BigMerkleTree<MNT6753Fr, MNT6753SmtPoseidonParameters, MNT6753PoseidonParameters>;

    #[test]
    fn compare_merkle_trees_mnt6_1() {
        use algebra::{
            fields::mnt6753::Fr, Field,
        };

        let num_leaves = 32;
        let mut leaves_to_process: Vec<OperationLeaf<MNT6753Fr>> = Vec::new();

        leaves_to_process.push(OperationLeaf { coord: Coord { height: 0, idx: 0 }, action: ActionLeaf::Insert, hash: Some(MNT6753Fr::from_str("1").unwrap()) });
        leaves_to_process.push(OperationLeaf { coord: Coord { height: 0, idx: 9 }, action: ActionLeaf::Insert, hash: Some(MNT6753Fr::from_str("2").unwrap()) });
        leaves_to_process.push(OperationLeaf { coord: Coord { height: 0, idx: 16 }, action: ActionLeaf::Remove, hash: Some(MNT6753Fr::from_str("3").unwrap()) });
        leaves_to_process.push(OperationLeaf { coord: Coord { height: 0, idx: 29 }, action: ActionLeaf::Insert, hash: Some(MNT6753Fr::from_str("3").unwrap()) });
        leaves_to_process.push(OperationLeaf { coord: Coord { height: 0, idx: 16 }, action: ActionLeaf::Remove, hash: Some(MNT6753Fr::from_str("3").unwrap()) });

        let mut smt = MNT6PoseidonSmt::new(num_leaves, String::from("./db_leaves") , String::from("./db_cache")).unwrap();
        smt.process_leaves_normal(leaves_to_process);

        //=============================================

        let mut leaves = Vec::new();
        leaves.push(MNT6753Fr::from_str("1").unwrap());
        for _ in 1..9 {
            let f = Fr::zero();
            leaves.push(f);
        }
        leaves.push(MNT6753Fr::from_str("2").unwrap());
        for _ in 10..29 {
            let f = Fr::zero();
            leaves.push(f);
        }
        leaves.push(MNT6753Fr::from_str("3").unwrap());
        for _ in 30..32 {
            let f = Fr::zero();
            leaves.push(f);
        }
        let tree = MNT6753FieldBasedMerkleTree::new(&leaves).unwrap();

        assert_eq!(tree.root(), smt.root, "Outputs of the Merkle trees for MNT4 do not match.");
    }

    #[test]
    fn compare_merkle_trees_mnt6_2() {
        use algebra::{
            fields::mnt4753::Fr,
            UniformRand,
        };

        let mut rng = XorShiftRng::seed_from_u64(9174123u64);
        let num_leaves = 32;
        let mut leaves = Vec::new();
        for _ in 0..num_leaves {
            let f = Fr::rand(&mut rng);
            leaves.push(f);
        }
        let tree = MNT4753FieldBasedMerkleTree::new(&leaves).unwrap();

        let mut smt = MNT4PoseidonSmt::new(num_leaves, String::from("./db_leaves") , String::from("./db_cache")).unwrap();
        let mut rng = XorShiftRng::seed_from_u64(9174123u64);
        for i in 0..num_leaves {
            let f = Fr::rand(&mut rng);
            smt.insert_leaf(
                Coord{height:0, idx:i},
                f,
            );
        }

        assert_eq!(tree.root(), smt.root, "Outputs of the Merkle trees for MNT4 do not match.");
    }

    #[test]
    fn compare_merkle_trees_mnt6_3() {
        use algebra::{
            fields::mnt4753::Fr, Field,
        };

        let num_leaves = 32;
        let mut leaves = Vec::new();
        leaves.push(MNT4753Fr::from_str("1").unwrap());
        for _ in 1..9 {
            let f = Fr::zero();
            leaves.push(f);
        }
        leaves.push(MNT4753Fr::from_str("2").unwrap());
        for _ in 10..29 {
            let f = Fr::zero();
            leaves.push(f);
        }
        leaves.push(MNT4753Fr::from_str("3").unwrap());
        for _ in 30..32 {
            let f = Fr::zero();
            leaves.push(f);
        }
        let tree = MNT4753FieldBasedMerkleTree::new(&leaves).unwrap();

        let mut smt = MNT4PoseidonSmt::new(num_leaves, String::from("./db_leaves") , String::from("./db_cache")).unwrap();
        smt.insert_leaf(Coord{height:0, idx:0}, MNT4753Fr::from_str("1").unwrap());
        smt.insert_leaf(Coord{height:0, idx:9}, MNT4753Fr::from_str("2").unwrap());
        smt.insert_leaf(Coord{height:0, idx:16}, MNT4753Fr::from_str("10").unwrap());
        smt.insert_leaf(Coord{height:0, idx:29}, MNT4753Fr::from_str("3").unwrap());
        smt.remove_leaf(Coord{height:0, idx:16});

        assert_eq!(tree.root(), smt.root, "Outputs of the Merkle trees for MNT4 do not match.");
    }
}