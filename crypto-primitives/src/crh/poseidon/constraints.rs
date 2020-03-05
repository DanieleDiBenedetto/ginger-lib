use algebra::{Field, PrimeField};
use crate::crh::poseidon::PoseidonParameters;
use std::marker::PhantomData;
use r1cs_core::{ConstraintSystem, SynthesisError};
use r1cs_std::fields::fp::FpGadget;
use r1cs_std::fields::FieldGadget;
use r1cs_std::bits::boolean::Boolean;
use r1cs_std::alloc::AllocGadget;
use r1cs_std::Assignment;
use r1cs_std::eq::ConditionalEqGadget;

pub struct PoseidonHashGadget
<
    ConstraintF: PrimeField,
    P:           PoseidonParameters<Fr = ConstraintF>,
>
{
    _field:      PhantomData<ConstraintF>,
    _parameters: PhantomData<P>,
}

impl<ConstraintF, P> PoseidonHashGadget<ConstraintF, P>
    where
        ConstraintF: PrimeField,
        P:           PoseidonParameters<Fr = ConstraintF>
{
    fn check_evaluation_gadget<CS: ConstraintSystem<ConstraintF>>(
        mut cs: CS,
        input: &[FpGadget<ConstraintF>],
    ) -> Result<FpGadget<ConstraintF>, SynthesisError>
    // Assumption:
    //     rate r = 2
    //     capacity c = 1
    //     t = 3
    {


        //let s1 = FpGadget::<ConstraintF>::alloc_hardcoded(cs.ns(||"alloc_zero"), val1)?;
        let zero = FpGadget::<ConstraintF>::zero(cs.ns(||"alloc_zero"))?;
        // state is a vector of 3 elements. They are initialized to zero elements
        let mut state = vec![zero.clone(), zero.clone(), zero.clone()];

        // calculate the number of cycles to process the input dividing in portions of rate elements
        let num_cycles = input.len() / P::R;
        // check if the input is a multiple of the rate by calculating the remainder of the division
        // the remainder of dividing the input length by the rate can be 1 or 0 because we are assuming
        // that the rate is 2
        let rem = input.len() % P::R;

        // apply permutation to all zeros state vector
        Self::poseidon_perm(cs.ns(|| "initial_poseidon_perm"),  &mut state)?;


        // index to process the input
        let mut input_idx = 0;
        // iterate of the portions of rate elements
        for i in 0..num_cycles {
            // add the elements to the state vector. Add rate elements
            for j in 0..P::R {
                state[j].add_in_place(cs.ns(|| format!("add_input_{}_{}", i, j)), &input[input_idx])?;
                input_idx += 1;
            }
            // for application to a 2-1 Merkle tree, add the constant 3 to the third state vector
            state[P::R].add_constant_in_place(cs.ns(||format!("add_constant_C2_{}",i)), &P::C2)?;
            // apply permutation after adding the input vector
            Self::poseidon_perm(cs.ns(|| format!("poseidon_perm_{}", i)),&mut state)?;
        }

        // in case the input is not a multiple of the rate process the remainder part padding a zero
        // in this case add C2 to state[2]
        //
        //   rem_input   0       C2
        // + state[0] state[1] state[2]
        //
        if rem != 0 {
            state[0].add_in_place(cs.ns(|| "poseidon_padding_add"), &input[input_idx])?;
            state[P::R].add_constant_in_place(cs.ns(||"add_constant_C2_last_chunk"), &P::C2)?;
            // apply permutation after adding the input vector
            Self::poseidon_perm(cs.ns(|| "poseidon_padding_perm"), &mut state)?;
        }

        // return the first element of the state vector as the hash digest
        Ok(state[0].clone())
    }

    fn mod_inv_sbox<CS: ConstraintSystem<ConstraintF>>(
        mut cs: CS,
        x: &mut FpGadget<ConstraintF>,
    ) -> Result<(), SynthesisError>
    {
        let b = Boolean::alloc(cs.ns(|| "alloc b"), || {
            let x_val = x.get_value().get()?;
            if x_val == ConstraintF::zero() {
                Ok(false)
            } else {
                Ok(true)
            }
        })?;
        let y = FpGadget::<ConstraintF>::alloc(cs.ns(|| "alloc y"), || {
            let x_val = x.get_value().get()?;
            if x_val == ConstraintF::zero() {
                Ok(x_val)
            } else {
                let inv = x_val.inverse().get()?;
                Ok(inv)
            }
        })?;
        cs.enforce(
            || "b=x*y",
            |lc| &x.variable + lc,
            |lc| &y.variable + lc,
            |lc| lc + &b.lc(CS::one(), ConstraintF::one()),
        );
        x.conditional_enforce_equal(cs.ns(|| "0=(1-b)*(x-y)"), &y, &b.not())?;

        *x=y;
        Ok(())
    }

    fn poseidon_perm<CS: ConstraintSystem<ConstraintF>>(
         mut cs: CS,
         state: &mut [FpGadget<ConstraintF>],
        ) -> Result<(), SynthesisError>
    {

        // index that goes over the round constants
        let mut round_cst_idx = 0;

        {
            // Add initial round constants

            for d in state.iter_mut() {
                let rc = P::ROUND_CST[round_cst_idx];
                (*d).add_constant_in_place(cs.ns(|| format!("add_constant_{}", round_cst_idx)), &rc)?;
                round_cst_idx += 1;
            }
        }

        // First full rounds
        for i in 0..P::R_F {

            // Apply the S-BOX to each of the elements of the state vector
            for (j, d) in state.iter_mut().enumerate() {
                Self::mod_inv_sbox(cs.ns(||format!("mod_inv_S-Box_1_{}_{}",i, j)), d)?;
            }

            // Perform the matrix mix
            Self::matrix_mix (cs.ns(|| format!("poseidon_mix_matrix_first_full_round_{}", i)), state)?;

            // Add the round constants to the state vector
            for d in state.iter_mut() {
                let rc = P::ROUND_CST[round_cst_idx];
                (*d).add_constant_in_place(cs.ns(|| format!("add_constant_1_{}", round_cst_idx)), &rc)?;
                round_cst_idx += 1;
            }

        }

        // Partial rounds
        for _i in 0..P::R_P {

            // Apply S-Box only to the first element of the state vector
            Self::mod_inv_sbox(
                cs.ns(||format!("mod_inv_S-Box_2_{}_{}",_i, 0)),
                &mut state[0]
            )?;

            // Perform the matrix mix
            Self::matrix_mix (cs.ns(|| format!("poseidon_mix_matrix_partial_round_{}", _i)), state)?;

            // Add the round constants to the state vector
            for d in state.iter_mut() {
                let rc = P::ROUND_CST[round_cst_idx];
                (*d).add_constant_in_place(cs.ns(|| format!("add_constant_2_{}", round_cst_idx)), &rc)?;
                round_cst_idx += 1;
            }

        }

        // Second full rounds
        // Process only to R_F -1 iterations. The last iteration does not contain a matrix mix
        for _i in 0..(P::R_F-1) {

            // Apply the S-BOX to each of the elements of the state vector
            for (j, d) in state.iter_mut().enumerate() {
                Self::mod_inv_sbox(cs.ns(||format!("mod_inv_S-Box_3_{}_{}",_i, j)), d)?;
            }

            // Perform the matrix mix
            Self::matrix_mix (cs.ns(|| format!("poseidon_mix_matrix_second_full_round_{}", _i)), state)?;

            // Add the round constants
            for d in state.iter_mut() {
                //let rc = MNT4753Fr::from_str(ROUND_CST[round_cst_idx]).map_err(|_| ()).unwrap();
                let rc = P::ROUND_CST[round_cst_idx];
                (*d).add_constant_in_place(cs.ns(|| format!("add_constant_3_{}", round_cst_idx)), &rc)?;
                round_cst_idx += 1;
            }
       }

        // Last full round does not perform the matrix_mix
        {
            // Apply the S-BOX to each of the elements of the state vector
            for (j, d) in state.iter_mut().enumerate() {
                Self::mod_inv_sbox(cs.ns(|| format!("mod_inv_S-Box_{}_{}", P::R_F-1, j)), d)?;
            }
        }

        Ok(())
    }

    // Function that does the mix matrix
    // Assumption: t = 3
    fn matrix_mix<CS: ConstraintSystem<ConstraintF>>(
        mut cs: CS,
        state: &mut [FpGadget<ConstraintF>],
    ) -> Result<(), SynthesisError>
    {

        //  s1'= (s1, s2, s3) * [ m11 m12 m13]
        //  s2'= (s1, s2, s3) * [ m21 m22 m23]
        //  s3'= (s1, s2, s3) * [ m31 m32 m33]
        // (s1, s2, s3) = (s1', s2', s3')

        // Check that the length of the state vector is t
        assert_eq!(state.len(), P::T);

        let m_11 = P::MDS_CST[0];
        let m_12 = P::MDS_CST[1];
        let m_13 = P::MDS_CST[2];

        // scalar multiplication for position 0 of the state vector
        let mut el_0 = state[0].mul_by_constant(cs.ns(||"partial_product_1_1"), &m_11)?;
        let elem_1 = state[1].mul_by_constant(cs.ns(||"partial_product_1_2"), &m_12)?;
        let elem_2 = state[2].mul_by_constant(cs.ns(||"partial_product_1_3"), &m_13)?;

        // sum of partial products
        el_0.add_in_place(cs.ns(|| "add_partial_product_1_2"), &elem_1)?;
        el_0.add_in_place(cs.ns(|| "add_partial_product_1_3"), &elem_2)?;

        // scalar multiplication for position 1 of the state vector
        let m_21 = P::MDS_CST[3];
        let m_22 = P::MDS_CST[4];
        let m_23 = P::MDS_CST[5];

        // scalar multiplication for position 1 of the state vector
        let mut el_1 = state[0].mul_by_constant(cs.ns(||"partial_product_2_1"), &m_21)?;
        let elem_4 = state[1].mul_by_constant(cs.ns(||"partial_product_2_2"), &m_22)?;
        let elem_5 = state[2].mul_by_constant(cs.ns(||"partial_product_2_3"), &m_23)?;

        // sum of partial products
        el_1.add_in_place(cs.ns(|| "add_partial_product_2_2"), &elem_4)?;
        el_1.add_in_place(cs.ns(|| "add_partial_product_2_3"), &elem_5)?;

        // scalar multiplication for the position 2 of the state vector
        let m_31 = P::MDS_CST[6];
        let m_32 = P::MDS_CST[7];
        let m_33 = P::MDS_CST[8];

        // scalar multiplication for position 2 of the state vector
        let mut el_2 = state[0].mul_by_constant(cs.ns(||"partial_product_3_1"), &m_31)?;
        let elem_7 = state[1].mul_by_constant(cs.ns(||"partial_product_3_2"), &m_32)?;
        let elem_8 = state[2].mul_by_constant(cs.ns(||"partial_product_3_3"), &m_33)?;

        // sum of partial products
        el_2.add_in_place(cs.ns(|| "add_partial_product_3_2"), &elem_7)?;
        el_2.add_in_place(cs.ns(|| "add_partial_product_3_3"), &elem_8)?;

        state[0] = el_0;
        state[1] = el_1;
        state[2] = el_2;

        Ok(())
    }

}


#[cfg(test)]
mod test {
    use super::super::rand::thread_rng;
    use r1cs_std::test_constraint_system::TestConstraintSystem;
    use crate::crh::poseidon::poseidon_original::poseidon_engine;
    use crate::crh::poseidon::parameters::MNT4753PoseidonParameters;
    use algebra::fields::mnt4753::Fr;
    use r1cs_std::fields::fp::FpGadget;
    use r1cs_std::alloc::AllocGadget;
    use r1cs_core::ConstraintSystem;
    use algebra::UniformRand;
    use crate::crh::poseidon::constraints::PoseidonHashGadget;

    type Mnt4FieldGadget = FpGadget<Fr>;

    #[test]
    fn crh_primitive_gadget_test() {

        let mut rng = &mut thread_rng();
        let mut cs = TestConstraintSystem::<Fr>::new();

        let mut vec_elem_4753 = Vec::new();
        let v1 = Fr::rand(&mut rng);
        let v2 = Fr::rand(&mut rng);
        vec_elem_4753.push(v1);
        vec_elem_4753.push(v2);

        let primitive_result = poseidon_engine::<MNT4753PoseidonParameters>(&mut vec_elem_4753);

        let v1_gadget = Mnt4FieldGadget::alloc(cs.ns(|| "alloc_v1"),|| Ok(v1)).unwrap();
        let v2_gadget = Mnt4FieldGadget::alloc(cs.ns(|| "alloc_v2"),|| Ok(v2)).unwrap();

        let mut vec_elem_gadget = Vec::new();
        vec_elem_gadget.push(v1_gadget);
        vec_elem_gadget.push(v2_gadget);

        let gadget_result =
            PoseidonHashGadget::<Fr, MNT4753PoseidonParameters>::check_evaluation_gadget(
                cs.ns(||"check_poseidon_gadget"),
                vec_elem_gadget.as_slice()).unwrap();

        println!("number of constraints total: {}", cs.num_constraints());

        assert_eq!(primitive_result, gadget_result.value.unwrap());
        assert!(cs.is_satisfied());
    }
}