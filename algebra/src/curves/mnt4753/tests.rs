use crate::{curves::{
    mnt4753::{
        G1Affine, G1Projective, G2Affine, G2Projective,
        MNT4,
    },
    tests::curve_tests,
    AffineCurve, PairingEngine,
}, biginteger::BigInteger768, fields::mnt4753::{fq::Fq, fq2::Fq2, fq4::Fq4, fr::Fr}, groups::tests::{
    group_test, compression_test, gt_compression_test
}, ProjectiveCurve, Field, PrimeField, ToBits, FromCompressedBits, SemanticallyValid};
use rand;
use std::ops::AddAssign;

#[test]
fn test_g1_projective_curve() {
    curve_tests::<G1Projective>();
}

#[test]
fn test_g1_projective_group() {
    let a: G1Projective = rand::random();
    let b: G1Projective = rand::random();
    group_test(a, b);
}

#[test]
fn test_g1_generator() {
    let generator = G1Affine::prime_subgroup_generator();
    assert!(generator.is_valid());
}

#[test]
fn test_g1_is_valid(){

    // Reject point with invalid x coordinate
    let p = G1Affine::new(
        Fq::new(BigInteger768([
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ])),
        Fq::from_repr(BigInteger768([
            0x717e3ffa2193697,
            0x411a81406abf8fc7,
            0x6c3f0710357570d0,
            0x9d999acc5cf81a11,
            0x81b6bf821df14a35,
            0x31135663344492a8,
            0x6da16e1624afd3a0,
            0x1c9a4d2e8eda6ba8,
            0xabe3b7346ad95eee,
            0xe39afac6814ca651,
            0xe0da6a8c4eb633d9,
            0xeed8b99aecdc,
        ])),
        false,
    );
    assert!(!p.is_valid());
    assert!(!p.x.is_valid());

    // Reject point with invalid y coordinate
    let p = G1Affine::new(
        Fq::from_repr(BigInteger768([
            0x4c0019d20f21bf0a,
            0x2412bb7c69103f8c,
            0xd837c81e51c23d86,
            0x25863118bf7cfccd,
            0xe33772d47fca8100,
            0xce263b8a45563538,
            0xd6d598765ee2b934,
            0x34e9e3c25ccc604f,
            0x4e3fdafc45d53a68,
            0xc92e2e4e5131ab8e,
            0x6da3e8856ccf21c3,
            0x89821510d8c7,
        ])),
        Fq::new(BigInteger768([
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ])),
        false,
    );
    assert!(!p.is_valid());
    assert!(!p.y.is_valid());

    //Reject point not belonging to curve
    let p = G1Affine::new(
        Fq::zero(),
        Fq::zero(),
        false,
    );
    assert!(!p.is_valid());
    assert!(!p.is_on_curve());

    // Reject point with invalid flags
    let mut p = G1Affine::zero();
    p.infinity = false;
    assert!(!p.is_valid());
    assert!(!p.is_infinity_flag_valid());

    let mut p: G1Projective = rand::random();
    while p.is_zero() {
        p = rand::random();
    }
    let mut p_affine = p.into_affine();
    p_affine.infinity = true;
    assert!(!p_affine.is_valid());
    assert!(!p_affine.is_infinity_flag_valid());

    // Accept valid point
    p_affine.infinity = false;
    assert!(p_affine.is_valid());

}

#[test]
fn test_g1_compression_decompression() {

    let even = G1Affine::new(
        Fq::from_repr(BigInteger768([
            0xf99bff3c256c04f0,
            0x3d6b06f9ad2e719d,
            0x23caf1a099fbff57,
            0x59b1d95d29ee4cab,
            0x5c68c6de94f80482,
            0x2f12567b30d1126b,
            0x52b9d710c49cf61e,
            0x3c57acbc06859f69,
            0xf2cbfbae4cca808a,
            0xe1ec5c19bd98638f,
            0x5a775231b500fd64,
            0x19beee8aae2b2,
        ])),
        Fq::from_repr(BigInteger768([
            0xe6a2ad4104991832,
            0x9d99a4bca7d41736,
            0x96cfdc5ffae430dc,
            0xbd0297adbec2c786,
            0xb04eed37d0cb1c3f,
            0xac2aeb03526fbe8a,
            0xf4f0d1e54394c0bb,
            0xeb93e7580b95e418,
            0x9d69ba42d9ea76bf,
            0x8a62f65f3500ebc7,
            0x56eb7a49f46d67e4,
            0x7ab01471e643,
        ])),
        false,
    );

    let odd = G1Affine::new(
      Fq::from_repr(BigInteger768([
          0x298807222674fefc,
          0x1242813cb96b8094,
          0x126d5a0db1b30eb,
          0xd8a19d8ffbf363c0,
          0x57e7610c00fb5761,
          0xabd86702edb9d1c8,
          0xd6539f3f7eb86f31,
          0x69fab29265443f1,
          0x2aec1c11b3d3c762,
          0x7f631cf6705e788f,
          0x575c4dc43a5a94ab,
          0x9ee12ac4d254,
      ])),
      Fq::from_repr(BigInteger768([
          0xf6ef0806b16c5e35,
          0xf895037990d4a025,
          0xad848aeca8a1b8,
          0x7aad4408b6befc89,
          0xd444a6fd3d7c8b1b,
          0x4b2be1c3b85792a9,
          0x2241ee1ddda9f812,
          0x6b9795fa6e987d16,
          0xe8e90d3fef6b271b,
          0x177df03c3274af1c,
          0xe71c5eac354f659e,
          0xdeca535c5f2f,
      ])),
        false,
    );

    compression_test::<G1Affine>(even, odd);

    //Test correct compression/decompression of a point with x = 0 coordinate
    let mut zero_bits = Fq::zero().write_bits();
    zero_bits.push(false); //Set infinity
    zero_bits.push(true); //Set parity
    assert!(G1Affine::decompress(zero_bits.clone()).is_ok());

    zero_bits.pop();
    zero_bits.push(false); //Change parity
    assert!(G1Affine::decompress(zero_bits.clone()).is_ok());
}

#[test]
fn test_g2_projective_curve() {
    curve_tests::<G2Projective>();
}

#[test]
fn test_g2_projective_group() {
    let a: G2Projective = rand::random();
    let b: G2Projective = rand::random();
    group_test(a, b);
}

#[test]
fn test_g2_generator() {
    let generator = G2Affine::prime_subgroup_generator();
    assert!(generator.is_valid());
}

#[test]
fn test_g2_is_valid(){

    // Reject point with invalid x coordinate
    let p = G2Affine::new(
        Fq2::new(
            Fq::new(BigInteger768([
                0xffffffffffffffff,
                0xffffffffffffffff,
                0xffffffffffffffff,
                0xffffffffffffffff,
                0xffffffffffffffff,
                0xffffffffffffffff,
                0xffffffffffffffff,
                0xffffffffffffffff,
                0xffffffffffffffff,
                0xffffffffffffffff,
                0xffffffffffffffff,
                0xffffffffffffffff,
            ])),
            Fq::from_repr(BigInteger768([
                0x786b46445f82e73c,
                0x7f65e493fad0e8b4,
                0x72f9e2017edbd8a0,
                0xa962e30713eac14b,
                0x4ff84799b321a106,
                0x8edd421b3377e583,
                0xaba7726f60af7957,
                0x333613a05885fc6b,
                0x6566cb2720173f7,
                0x2fbd93f05fb4aafb,
                0x5c36413ccc1c397b,
                0x101e0d7f5c50b,
            ])),
        ),
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0xe7542befc7c77e7e,
                0x3d532adf08c50ee,
                0x7931d21d45cf5b88,
                0xf3facf495cde403,
                0x6cb921a14e9a4f4f,
                0xa2c6fa913de0db27,
                0x9cd5563862ee6b52,
                0x609c608ee22298d1,
                0xc61a7826940542f7,
                0xa62753b7c5522ed8,
                0x8944940494f84bd9,
                0x134882728983e,
            ])),
            Fq::from_repr(BigInteger768([
                0xd5ae2443316eca5e,
                0x1e5d2cab9ea75b61,
                0x9c598bb3764d1f4a,
                0x8664602317bb85ca,
                0xeeb80880a81b30dd,
                0x7fa2cb7313ad08af,
                0xb8fa25436f268402,
                0xd0b2fb568b2db00,
                0xf85d0eda012e353e,
                0xddcd8a006eaad8b1,
                0x22349e5f59a72ea6,
                0x147db456d4e50,
            ])),
        ),
        false,
    );
    assert!(!p.is_valid());
    assert!(!p.x.is_valid());

    // Reject point with invalid y coordinate
    let p = G2Affine::new(
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0xcd3192cf5e7bfd5f,
                0x6cbe38e97527e8af,
                0xdd06c276b9a60b50,
                0xcba1ce70e3002cf2,
                0x4585e29519699027,
                0xd3a145616a69bffe,
                0xd00683c57b1918c2,
                0xf186e5d5c72154c0,
                0xa2d24f00463b7065,
                0xba25111f5a5085f3,
                0xcc0093f39b311579,
                0xaac25dd8a401,
            ])),
            Fq::from_repr(BigInteger768([
                0x786b46445f82e73c,
                0x7f65e493fad0e8b4,
                0x72f9e2017edbd8a0,
                0xa962e30713eac14b,
                0x4ff84799b321a106,
                0x8edd421b3377e583,
                0xaba7726f60af7957,
                0x333613a05885fc6b,
                0x6566cb2720173f7,
                0x2fbd93f05fb4aafb,
                0x5c36413ccc1c397b,
                0x101e0d7f5c50b,
            ])),
        ),
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0xe7542befc7c77e7e,
                0x3d532adf08c50ee,
                0x7931d21d45cf5b88,
                0xf3facf495cde403,
                0x6cb921a14e9a4f4f,
                0xa2c6fa913de0db27,
                0x9cd5563862ee6b52,
                0x609c608ee22298d1,
                0xc61a7826940542f7,
                0xa62753b7c5522ed8,
                0x8944940494f84bd9,
                0x134882728983e,
            ])),
            Fq::new(BigInteger768([
                0xffffffffffffffff,
                0xffffffffffffffff,
                0xffffffffffffffff,
                0xffffffffffffffff,
                0xffffffffffffffff,
                0xffffffffffffffff,
                0xffffffffffffffff,
                0xffffffffffffffff,
                0xffffffffffffffff,
                0xffffffffffffffff,
                0xffffffffffffffff,
                0xffffffffffffffff,
            ])),
        ),
        false,
    );
    assert!(!p.is_valid());
    assert!(!p.y.is_valid());

    //Reject point not belonging to curve
    let p = G2Affine::new(
        Fq2::zero(),
        Fq2::zero(),
        false,
    );
    assert!(!p.is_valid());
    assert!(!p.is_on_curve());

    // Reject point with invalid flags
    let mut p = G2Affine::zero();
    p.infinity = false;
    assert!(!p.is_valid());
    assert!(!p.is_infinity_flag_valid());

    let mut p: G2Projective = rand::random();
    while p.is_zero() {
        p = rand::random();
    }
    let mut p_affine = p.into_affine();
    p_affine.infinity = true;
    assert!(!p_affine.is_valid());
    assert!(!p_affine.is_infinity_flag_valid());

    // Accept valid point
    p_affine.infinity = false;
    assert!(p_affine.is_valid());

}

#[test]
fn test_g2_compression_decompression() {
    let even = G2Affine::new(
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0xcd3192cf5e7bfd5f,
                0x6cbe38e97527e8af,
                0xdd06c276b9a60b50,
                0xcba1ce70e3002cf2,
                0x4585e29519699027,
                0xd3a145616a69bffe,
                0xd00683c57b1918c2,
                0xf186e5d5c72154c0,
                0xa2d24f00463b7065,
                0xba25111f5a5085f3,
                0xcc0093f39b311579,
                0xaac25dd8a401,
            ])),
            Fq::from_repr(BigInteger768([
                0x786b46445f82e73c,
                0x7f65e493fad0e8b4,
                0x72f9e2017edbd8a0,
                0xa962e30713eac14b,
                0x4ff84799b321a106,
                0x8edd421b3377e583,
                0xaba7726f60af7957,
                0x333613a05885fc6b,
                0x6566cb2720173f7,
                0x2fbd93f05fb4aafb,
                0x5c36413ccc1c397b,
                0x101e0d7f5c50b,
            ])),
        ),
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0xe7542befc7c77e7e,
                0x3d532adf08c50ee,
                0x7931d21d45cf5b88,
                0xf3facf495cde403,
                0x6cb921a14e9a4f4f,
                0xa2c6fa913de0db27,
                0x9cd5563862ee6b52,
                0x609c608ee22298d1,
                0xc61a7826940542f7,
                0xa62753b7c5522ed8,
                0x8944940494f84bd9,
                0x134882728983e,
            ])),
            Fq::from_repr(BigInteger768([
                0xd5ae2443316eca5e,
                0x1e5d2cab9ea75b61,
                0x9c598bb3764d1f4a,
                0x8664602317bb85ca,
                0xeeb80880a81b30dd,
                0x7fa2cb7313ad08af,
                0xb8fa25436f268402,
                0xd0b2fb568b2db00,
                0xf85d0eda012e353e,
                0xddcd8a006eaad8b1,
                0x22349e5f59a72ea6,
                0x147db456d4e50,
            ])),
        ),
        false,
    );

    let odd = G2Affine::new(
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0x456744b2be8dbb17,
                0xecd53f9445858509,
                0xc8ef07e1ef24874b,
                0xe58bd001c96adf30,
                0xf5013e562055d5c4,
                0x91bcc876c246b01a,
                0xe99bac985d37109d,
                0xecc18710b80145ea,
                0xf81630b88849765d,
                0xf973901c825a4c9,
                0xf4065981d8af931e,
                0xc8c242e8774d,
            ])),
            Fq::from_repr(BigInteger768([
                0xbbe4e0cfa71026bc,
                0x7e57a663d9e42af9,
                0x5723ecf7d136bfe3,
                0x3d12606f04e325ba,
                0x2e9974c843fbd1e5,
                0x74186977f70f1557,
                0x274833a0fefcdc64,
                0xaa1029956b28c037,
                0xece7ab101af54218,
                0x712c615c6e235a04,
                0xd41e5306f6127f87,
                0x1b27e4bd7cbb7,
            ])),
        ),
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0xb8965e86840dd0e3,
                0x508bd0abf330deda,
                0x47376bb9625954f7,
                0x6ec582aa92574a15,
                0x2a1199db2cc4c995,
                0x59d0ce26672bf02c,
                0x1ae2ef282425bb0e,
                0xb08ad562cdac85e2,
                0x4ed8fb35c6197948,
                0x57f6c2a8d65b8391,
                0xc1f697c75d1415d6,
                0x126d1d01813f6,
            ])),
            Fq::from_repr(BigInteger768([
                0xd1914aadaa004e87,
                0x224c5a1af4b9d12a,
                0x28e91e3b461f0fa9,
                0xf71648756c554581,
                0xc37b9d797f44c663,
                0xb2e3e3b0ccdec02c,
                0x81fe2d79a55ee92,
                0xa0d16f9d5fdb5171,
                0x52f8c2e0c3fc5d06,
                0xdda69e868f2d328f,
                0x21ea99caf9bd207c,
                0x884a49271323,
            ])),
        ),
        false,
    );

    compression_test::<G2Affine>(even, odd);
}

#[test]
fn test_bilinearity() {

    let a = G1Projective::new(
        Fq::from_repr(BigInteger768([
            0x73aa4fa8b4cf832e,
            0xf6a20073ec5337fe,
            0xe8f3e58577abc4e7,
            0x36d61a68c4cbb95b,
            0x40416854fa978685,
            0x265af69871df33f4,
            0x93f9daa280d7b196,
            0xb61de76c321e6bb5,
            0xda4f508f7c892c6a,
            0x6280bfaf4e4d70c8,
            0x175c61d672e9ab0,
            0x161c781730586,
        ])),
        Fq::from_repr(BigInteger768([
            0x7db80364f4b2d45d,
            0xb4773092c4dacfd9,
            0xfd3b9f2004d1378a,
            0x5f83ad886a4eb74b,
            0x53a78ceeba4bb2d7,
            0x997db9f866f9cc86,
            0xac73f4c804bf252f,
            0xf39219de12f4e5de,
            0x260331a4c801f26d,
            0x6316779797551fc5,
            0xad8b27a570e82575,
            0x713c9a5182c1,
        ])),
        Fq::from_repr(BigInteger768([
            0x5182eb8a2e38a933,
            0x197bf6c58c637f58,
            0x326ad45b0bbe5faa,
            0x1e87ae4a69a5b392,
            0x9fb398ee38569f3b,
            0x7c03785f38cd16c8,
            0x39cedf3d32acdf65,
            0x962167ddaadc4b35,
            0x507438bea3186c81,
            0xc1b816ee9214a3da,
            0xb2c6f0be99d43f9f,
            0xfb8ad6448db7,
        ])),
    );

    let b = G2Projective::new(
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0x212bdee61bd64c25,
                0x6ef4a70acdfe2aed,
                0x13ad51950a78e9e,
                0x3f83dc4f1763db79,
                0xc0543ff37a10b092,
                0xb181f3a2f30bc152,
                0x29d5b7838de2b79b,
                0xc806a8572bbc221b,
                0x10388571f57a4081,
                0x272c506912f798ec,
                0x725303c28efb0b6c,
                0x73bef63b2450,
            ])),
            Fq::from_repr(BigInteger768([
                0x87b373d53c256b5c,
                0xb396a4fd859f1b5d,
                0x45723ec8ed69e363,
                0x8a870ee7aace411e,
                0xd921d6bcbce1594,
                0xcc3901ac91e0dbb6,
                0xef9cd7649614736b,
                0x5f5058c458c88789,
                0x872a7982eaab973,
                0x9372416d55a496b3,
                0x12b2f8dc6b2a3e1b,
                0x1735779cd8f93,
            ])),
        ),
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0xa938f18b73bd1d69,
                0xd74d8abbd25a19ef,
                0xfa8c7ad66be6bdc3,
                0xdf3328985023ba38,
                0xe00973ba49f07643,
                0xc70e3a623e1d5a5a,
                0xc3f49b792a0f3ac7,
                0x61b0a35f16db9042,
                0xa64b33362f4086c,
                0x4b0d657a5bbbf785,
                0x1ea422bebb1bb410,
                0x900b0408c3bb,
            ])),
            Fq::from_repr(BigInteger768([
                0x16b615452dd8a39a,
                0x67cd711e41b41b08,
                0xab19d17d0a9f47c3,
                0xbfaec25d8b254c11,
                0x79f31dfb7014c0ff,
                0x872d08e7dd566561,
                0xbd835abc04a17c47,
                0xc70bc09268bdce30,
                0x257cfaf12aef6552,
                0xd7b3aff45481cf0c,
                0x51ae5460f82450e4,
                0x1746155d1e10a,
            ])),
        ),
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0x5faea8d460c40219,
                0x985aec5e84a6e671,
                0x5a3532d4108bdc01,
                0xbad727383937b154,
                0x3f4223fd6a99cfe4,
                0x65444db13d524378,
                0x7eecc8b800abd16d,
                0x31151f62e2fd9878,
                0x9a3f7924d3736709,
                0x9ccd668b736771b1,
                0xf8776b699d97c58d,
                0xc5347f83cb35,
            ])),
            Fq::from_repr(BigInteger768([
                0x295e890f3b121ffd,
                0x6589a8e773911a99,
                0xce3c6ef5ae53eb60,
                0x74584e98e49bfc03,
                0x49b32063feb1a1d7,
                0xa068fafa575e98a7,
                0x3906e36283b23e2,
                0xb58dfeca3f436e12,
                0x1f79b6a80b0a45ae,
                0xaf49bcd6812cac4d,
                0xbb2e388cd1b712a6,
                0x186d11afaedeb,
            ])),
        ),
    );

    assert_eq!(MNT4::pairing(a, b), Fq4::new(
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0x402c1cfcda2faf66,
                0xf6c7304f2122f8aa,
                0xd4b9967cb518343c,
                0x9e53b3e7641ef2f1,
                0x19a50acd733e1d2a,
                0xd6379b98506bfec0,
                0x8c62f80413fb292d,
                0x2abbeaf72d4d0ae3,
                0x4371afba748f7323,
                0xfb1dc6fe5f878bd9,
                0xecba1152795ccec9,
                0x17a073784408e,
            ])),
            Fq::from_repr(BigInteger768([
                0xc8f0d77ac269818d,
                0x16159718aaddf6b4,
                0x836adebc2c3dca80,
                0x9e62cb6d0ea92d11,
                0xe3c1623bfe8be7d3,
                0x93effa98bfdcd840,
                0x924f8243ccd9777,
                0xc01752ae12a2226b,
                0xa0c91443314f26ba,
                0x4b86b7727bfeb5cf,
                0xbe176e053bb5f896,
                0x18ea90d330e30,
            ])),
        ),
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0xe6eb9d8b79c51b25,
                0x315aa673f71bdbdd,
                0xddf71960cc943d17,
                0xbe9b960873b6ec40,
                0x344eb4c7b642a2cc,
                0x5e15749faf046966,
                0xf2f068f5c06bf7bd,
                0xeb244db2d00ca3d,
                0xf24560453af9e6b0,
                0x7313e48f2b23e781,
                0xdee9567a39adb3b6,
                0x5c91032a5add,
            ])),
            Fq::from_repr(BigInteger768([
                0x7ef5c16eb20441d4,
                0xc360b3e98c43be1c,
                0xabe5ab03d478dc53,
                0x18354f21b0af5d09,
                0xabec396659da3195,
                0xb8a4594f91bf03b2,
                0x3619f92f87324038,
                0xe725218241105863,
                0xcc03bdd0d8f636c4,
                0x6db6695c012d9c5e,
                0x5c14cdb35a6edd8f,
                0x1406d4fd7fcd0,
            ])),
        ),
    ));

    let a: G1Projective = rand::random();
    let b: G2Projective = rand::random();
    let s: Fr = rand::random();

    let sa = a * &s;
    let sb = b * &s;

    let ans1 = MNT4::pairing(sa, b);
    let ans2 = MNT4::pairing(a, sb);
    let ans3 = MNT4::pairing(a, b).pow(s.into_repr());

    assert_eq!(ans1, ans2);
    assert_eq!(ans2, ans3);

    assert_ne!(ans1, Fq4::one());
    assert_ne!(ans2, Fq4::one());
    assert_ne!(ans3, Fq4::one());

    assert_eq!(ans1.pow(Fr::characteristic()), Fq4::one());
    assert_eq!(ans2.pow(Fr::characteristic()), Fq4::one());
    assert_eq!(ans3.pow(Fr::characteristic()), Fq4::one());
}

#[test]
fn test_gt_compression(){
    let even = Fq4::new(
        Fq2::new(
           Fq::from_repr(BigInteger768([
               0xd96a230bc7704e70,
               0xa72f0322390e389a,
               0xa21eaf1d7b0bd422,
               0xdafa420c44e8fc7f,
               0xa86860e419cf404,
               0x90b13c03cd5adc5d,
               0x5de378e3fa270986,
               0x9f61f6d2c77e51eb,
               0x4b66e983144a9cd7,
               0x5636ef040a8f76f2,
               0x9ce9f6d852eb88d4,
               0x1b370b90e36dc,
           ])),
           Fq::from_repr(BigInteger768([
               0xff11ebd7b4b68af2,
               0x821c77749ea7d163,
               0x66d9f563048ccd41,
               0x896ffff75e0497c6,
               0xbb79afca8f854a5d,
               0x2454181dd568edde,
               0x4e99f7708b4609ac,
               0xed0d14da98b5a6fb,
               0x9a9e76fd9ba121e,
               0xd0995d3c899c6720,
               0xeb1206daf656d48f,
               0x159e65acb07ab,
           ])),
        ),
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0x4d22f2ba0c1b60cf,
                0xee8f48b783eb56c2,
                0x7cca465ac7f4714e,
                0xa5558ed13cd5f825,
                0x61a7def725192a30,
                0xa924dd42cb05517a,
                0x5a10987421894de3,
                0xc6a7abc1af1a3a5e,
                0xa5aca857fd6d5b0b,
                0x20f08311cd3d2876,
                0x4e573b3035b9241,
                0x1c8e6b81fae,
            ])),
            Fq::from_repr(BigInteger768([
                0x81e5374e0c7b19b8,
                0xfa684c0a6d680b83,
                0x1f4004e7cb96abe3,
                0x4ed0bab266d80e67,
                0x773f99be7632257d,
                0xb1a80406a8d3a44d,
                0xb771cd7a1bf6591b,
                0x5e21a2060462025b,
                0xcb6492fc43cd56f3,
                0xc1be58f7b8353e41,
                0x7ae25e9d427abd05,
                0x6961b23e2c13,
            ])),
        ),
    );

    let odd = Fq4::new(
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0x748e5389bec85d58,
                0x1a20edc17fb399dd,
                0xd83c1c41d58d9ca1,
                0x235f6338741f6571,
                0x2d547f893dca1fc1,
                0xef690dee9a29f09b,
                0x19c9504943555934,
                0x3ca188eac4f6f913,
                0xa33ac3632da47eaf,
                0x9b59580f494a8248,
                0x585ba26820ef9787,
                0xd5ef38abeb86,
            ])),
            Fq::from_repr(BigInteger768([
                0x5e3d21535ca78e93,
                0x1c9a46335817b761,
                0xc52c570a95e424b1,
                0xe8f37867cab6fef,
                0x65bc27c56f08d449,
                0xb523c446e6117eeb,
                0x8cc83ec1ea1b1d86,
                0xeb87b331e37696c9,
                0xada589e8e2d3b724,
                0x9940a3041c066a49,
                0x6e10bac06013fc45,
                0x13f291e853add,
            ])),
        ),
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0xdb52b0436e0860e0,
                0x9592684b4b1643d2,
                0xcbd8aa405ba3e87b,
                0xe992d731003be6a8,
                0x9461ba77684fc1bf,
                0x1b5e74c0fe5e3ac4,
                0xd18d96a0d1afa659,
                0xce0e2da0a8c409a3,
                0x4394347b9ced1dbd,
                0x7a41b0c133f1d2f6,
                0x984cf0632afc9ba,
                0x18be95e126b67,
            ])),
            Fq::from_repr(BigInteger768([
                0x45cf724c7263f1ea,
                0xe4c3de5bf1f4685f,
                0xae6a4f38a7b0504a,
                0xece95374c1d73fa,
                0x716f172d5df9c8c3,
                0xcac39a53ea8a67d,
                0x8a0871212b90ed0a,
                0xb1555262cab7c2be,
                0xeb199d08585d401d,
                0xc746bd450cd53e2a,
                0xc189c8ca7056ddd6,
                0x11c5ffcecd4cc,
            ])),
        ),
    );

    gt_compression_test::<Fq4>(even, odd);
}

#[test]
fn test_g1_addition_correctness() {
    let mut p = G1Projective::new(
        Fq::from_repr(BigInteger768([
            0xdfc2b7cbc7c68dd3,
            0xda35f108daf8530a,
            0xed1046ac66215fc2,
            0x456d3bec410beaa6,
            0x83c63b83fe368eb7,
            0x7f3cf1cdbb8d1853,
            0x3e750f1448b7d6d1,
            0x73c851e84a248dd4,
            0x54c871325cf89d71,
            0xb12d77db967730e8,
            0x6c13fdb8114e2ee5,
            0x178dc471842c9,
        ])),
        Fq::from_repr(BigInteger768([
            0xd4d304c070359df4,
            0x5adaedd2f9769957,
            0x9de60988567a0d8c,
            0x1597b5a2f48619ed,
            0xf12ac0e35580012b,
            0xec8d60978bf1abf3,
            0xfef31c938dc5e3ec,
            0x92afc1446830abca,
            0xbf2f83c9f917e43b,
            0x989cd6d6e7be1543,
            0xa0f2fcb4e8bbdaf0,
            0xf93208b6420f,
        ])),
        Fq::from_repr(BigInteger768([
            0xda8eb0b8eef48fac,
            0x87b1d0180184f6c0,
            0xe6a04246ed619d42,
            0x501ee89d33c211de,
            0xcfe58f6f87258b40,
            0x742345656f9fd427,
            0xe8f4d82210ea7d4e,
            0xa51004b4f76e2fc2,
            0xc87d9dae17bfc00c,
            0x9e38fcd739c6212d,
            0x7c5aa6a69ea22272,
            0x134d799c12c4e,
        ])),
    );

    p.add_assign(&G1Projective::new(
        Fq::from_repr(BigInteger768([
            0xd8b796054c3c07aa,
            0x7a2d262560ad2558,
            0xe9fb791faa62f5e4,
            0x5efb0ed78efd43c4,
            0xe7b524c5b6e01e61,
            0x526a03896c7f0c9f,
            0x5a9f513428a2d469,
            0xe27368abe47ec9e6,
            0xc1b7389ed619aac1,
            0x549f36555acde762,
            0xf7a4799366140f73,
            0xb530c14e43de,
        ])),
        Fq::from_repr(BigInteger768([
            0xcab7d6b352fa4d19,
            0xe9f3586d0007f1a6,
            0xdd1eb2c0c9af5d0,
            0x1361a6325decd10a,
            0x6e4f39f933bc89d5,
            0xe601021834b48b43,
            0x6f34ae367a105a4d,
            0xf1d34502f2a97dae,
            0x9c21874f2ddb6af9,
            0xf6cafeb3010bb13f,
            0x53566dd8c94a881,
            0x1d645de11625,
        ])),
        Fq::from_repr(BigInteger768([
            0xfb60f671d178d6e7,
            0xb785b67ab21fea76,
            0xfcb57401fe0ffe84,
            0xa12d2ea0964e19a5,
            0x2fab37c250e2a2fa,
            0x868711fb5eaad3f,
            0xb1868139f022ff77,
            0xa1b225670e5bdcdd,
            0x6dcdf5cfbcec9f85,
            0xbbcf4300efa53b07,
            0x695973d4beef9e99,
            0x6d46f5bcabde,
        ])),
    ));

    let p = G1Affine::from(p);

    assert_eq!(
        p,
        G1Affine::new(
            Fq::from_repr(BigInteger768([
                0x4c0019d20f21bf0a,
                0x2412bb7c69103f8c,
                0xd837c81e51c23d86,
                0x25863118bf7cfccd,
                0xe33772d47fca8100,
                0xce263b8a45563538,
                0xd6d598765ee2b934,
                0x34e9e3c25ccc604f,
                0x4e3fdafc45d53a68,
                0xc92e2e4e5131ab8e,
                0x6da3e8856ccf21c3,
                0x89821510d8c7,
            ])),
            Fq::from_repr(BigInteger768([
                0x717e3ffa2193697,
                0x411a81406abf8fc7,
                0x6c3f0710357570d0,
                0x9d999acc5cf81a11,
                0x81b6bf821df14a35,
                0x31135663344492a8,
                0x6da16e1624afd3a0,
                0x1c9a4d2e8eda6ba8,
                0xabe3b7346ad95eee,
                0xe39afac6814ca651,
                0xe0da6a8c4eb633d9,
                0xeed8b99aecdc,
            ])),
            false,
        )
    );
}

#[test]
fn test_g1_doubling_correctness() {
    let mut p = G1Projective::new(
        Fq::from_repr(BigInteger768([
            0x6064ee639b9adce5,
            0x1149f14300102ddd,
            0x395f28b5c8101bd0,
            0xa764e4bdd6b33c5a,
            0x51e645dfb580ecac,
            0x2ca75c22f9d5b856,
            0x4314a9a2a058df54,
            0x75886b456ad32bfa,
            0x3f4c758a65245bdb,
            0x49129d70da6fe6a8,
            0xbc4dac6eb4f07c3b,
            0x47acb9975aa8,
        ])),
        Fq::from_repr(BigInteger768([
            0xaa39a144b0311d5e,
            0x89f04b3a9adebdaf,
            0xd32e9cc742b76970,
            0x6672d161ca75793e,
            0x6e8c03b3f80c227c,
            0xc32a6f51615d8fba,
            0xcbad4d6317f1cf55,
            0x1eafa5de19fc6007,
            0xfd55c1cf34af1159,
            0xb2522dd8a5b9e91b,
            0x540709a8841364c3,
            0x50e2d88b5db9,
        ])),
        Fq::from_repr(BigInteger768([
            0x624f5f5b6e628648,
            0xb43340d2bb9406b4,
            0xd997cb8475d5b4cf,
            0xc22fdbdc06ba16e8,
            0x92220503c51b8328,
            0x42916d7ff8dd732,
            0x6d3df7f377a02d2c,
            0x5b3e1058294a7493,
            0x653fd02a7f2ab972,
            0x111806291f570f83,
            0x800bce7fd996bb00,
            0x938c7238a9a7,
        ])),
    );

    p.double_in_place();

    let p = G1Affine::from(p);

    assert_eq!(
        p,
        G1Affine::new(
            Fq::from_repr(BigInteger768([
                0x7548af158fa3fc51,
                0x6cd80c3910403c9e,
                0x6c9f15e06b5ba60d,
                0xb6a754b513529f07,
                0x23c496e83a606680,
                0x21ce1759ba83590c,
                0xb407ab047a9edef1,
                0x6fd97e8ab8d36ab6,
                0x6d82dcd641f777e4,
                0x6caf6c3a77a44722,
                0xbbfc52c0db6b150f,
                0x1b5aab811e031,
            ])),
            Fq::from_repr(BigInteger768([
                0xb23c84ab63b585ed,
                0xefb84ff2c341b21,
                0x86b0efe06b5887f1,
                0x49b1982bc6146cea,
                0x72c68986a18645ae,
                0x7eee2d44d74827a9,
                0xe03d44233741d59d,
                0x285deaac6cec108d,
                0xe4aea4c6b9967a8d,
                0x9c83e0356b9eeedd,
                0xbbeb2089d3321306,
                0x1bffa53113921,
            ])),
            false,
        )
    );
}

#[test]
fn test_g1_scalar_multiplication(){

    let a = G1Affine::new(
        Fq::from_repr(BigInteger768([
            0x925c1f040aed511c,
            0x5a855427a50c739e,
            0x7ab9b2e57d5f3a13,
            0x513a6ec73171e05b,
            0x6b6a8244ed00762e,
            0x87d0a8427d0e5d36,
            0x417a733b306444eb,
            0xcaae9edbde381d27,
            0x5124f71e848677c3,
            0x47f710cb2a44cc08,
            0x637820bb0dbcadc4,
            0xeb54b306da09,
        ])),
        Fq::from_repr(BigInteger768([
            0x1228573b93928314,
            0xbef47d91a144ae9c,
            0x896dd71c348196f9,
            0x769373819a2cef8a,
            0x65cee2c1ca362519,
            0x1603c65c14b30c1d,
            0x8b56003559d55972,
            0x9f4e3fe9dbbd6220,
            0x213fcc2184a77813,
            0xe56d2370e454f40c,
            0xc190214c5f7852d2,
            0xfa70dc17467,
        ])),
        false
    );

    let scalar = Fr::from_repr(BigInteger768([
        0x56d6335b0db8aabc,
        0xead28ee558ffe882,
        0x1d5d812f693d85e,
        0x6a8759a07487aefa,
        0xc0c017fad83d37d8,
        0xdd1d91c4f3e3e08a,
        0xb430af48b77f22b4,
        0x4542b11e681a7fba,
        0x8bc1c9779783bcb0,
        0x12a9272e34a41ef6,
        0x3d2a16c493861827,
        0x1ae02b26d23fc,
    ]));


    assert_eq!((a.mul(scalar)).into_affine(),
       G1Affine::new(
           Fq::from_repr(BigInteger768([
               0xa3d0ff067624b1d0,
               0x72664096e2577c12,
               0xa5c0016be5fb83df,
               0x5588f243586b074c,
               0x396f6f744d0f68c8,
               0xdaee682fe15fee44,
               0xacd54a6087292adf,
               0x5332945f07fe151e,
               0x1bebf1a88348a53f,
               0x7587d62f9ac2bf4f,
               0x71d34fed05742694,
               0xc20022e36488,
           ])),
           Fq::from_repr(BigInteger768([
               0x672f790a2f76b3c,
               0x7cb36ead4c6fc730,
               0x2d61577762e758c8,
               0xe7750a8d982c291a,
               0x9fa3e61e2f101365,
               0x861251642d4c395e,
               0x84a9c031a9904727,
               0x18b5e27eb7de60e8,
               0x26c981935aa683db,
               0x3b9efd259ef81353,
               0x17f763c20d5ac84b,
               0x127cf79606e7c,
           ])),
           false
       )
    );
}

#[test]
fn test_g1_affine_projective_conversion() {

    let a = G1Projective::new(
        Fq::from_repr(BigInteger768([
            0xb0ed63c4d24e8a1c,
            0x50bb6ed9a0862dcb,
            0x8c6c55ec0725bb6f,
            0x7a6117f051cd5547,
            0x64d4b0df25a12962,
            0x91ab55890e2526e7,
            0x428f5f3aada89b,
            0xbb4a3b186aba610a,
            0x36700cb6f89b3ee1,
            0x930d401e36897ef5,
            0x906ff098de3c93f2,
            0x19e3e488e71ab,
        ])),
        Fq::from_repr(BigInteger768([
            0xc50ad9cddc4166f4,
            0xde424a828125a247,
            0x1e305c4683b1b04,
            0x82956035d685ec86,
            0xe7f3d4aa35f3f260,
            0x3d0c86f95c30e08b,
            0xc81563979c73ac11,
            0xdaa4b524232f3b91,
            0xb82d239f07291d52,
            0x3af3efc3be33fa0d,
            0x5e32f03921ce21e9,
            0x1c0e5725f0478,
        ])),
        Fq::from_repr(BigInteger768([
            0x8b4fd5b8fd427b06,
            0x7d96a019653440df,
            0xd1939b3e70a64dfe,
            0x1c76c8553d664fe6,
            0xcf97c439c6627269,
            0x9732c7d39d91a667,
            0x8189ec69de56614c,
            0x1689e27635db4a89,
            0x97327af8bdad4b4,
            0xc3c8289fdb7b8219,
            0xb69d9535a1a29db4,
            0x28d4d4d8cd89,
        ])),
    );

    let a_a = a.into_affine();
    assert_eq!(a_a, G1Affine::new(
        Fq::from_repr(BigInteger768([
            0x39dd21ca6cbfad4e,
            0x64377a8f5a31e9f8,
            0xe6e7270485da014e,
            0x1a6f5a1fbe80813a,
            0x60a9a80a0e3a087,
            0x7b3c779f982b8c98,
            0x6179fcdf7893edd4,
            0x605b3e41f254b925,
            0x4d1e2d030ab467e3,
            0xa9283d87bf549304,
            0xe34151514840f804,
            0x1436619e846e,
        ])),
        Fq::from_repr(BigInteger768([
            0xf06f708098de8eea,
            0xfdb4381ac7c97df,
            0x3275b4aed28fc16,
            0x3dae26a63f5f2fc0,
            0xf88932458695b60b,
            0x5406864079c3a5b3,
            0x757d70bd6d0d4ee,
            0xa3b5eb573b241e0e,
            0x9874335300ece9bd,
            0xe1dd8f781c582459,
            0xe981c3e4590b9847,
            0x1bdf789b4fc48,
        ])),
        false,
    ));

    assert_eq!(a_a.into_projective(), a);
}

#[test]
fn test_g2_addition_correctness() {
    let mut p = G2Projective::new(
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0x5f3223a07f366969,
                0x2a2ec7caeb288ce9,
                0x17aab769f779b5a9,
                0x6f6b3a2e08d4bac9,
                0xfb76d77ef5383397,
                0x25a4a9f1ad927d94,
                0x8205c8e3dda818b9,
                0xefbd0dfae72f83da,
                0x7cb4a2c1e95a3983,
                0xdc890797f8f6a8de,
                0x14a99e12c4d27d9a,
                0x2f65a6298d59,
            ])),
            Fq::from_repr(BigInteger768([
                0x9dda0049bcccf9e1,
                0xac33968d1278a69b,
                0x7f4303a18cf004ec,
                0xef41161159848b35,
                0x933cbd1c68fc9d0b,
                0x3fe12e20b4c2a325,
                0x4429d610856b837d,
                0xb1eaffd8ff610f97,
                0x84b431125114b908,
                0x5c5bc1e9a819fdf0,
                0x4e4e397f9b60e231,
                0x420182fa92d7,
            ])),
        ),
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0x2a14a07b3d0962bf,
                0x2b7404e772f171f5,
                0x91328236e6f1dfde,
                0xa2be3582f495644a,
                0xe52f9290022d5951,
                0x843f9dbae42c5516,
                0xf3ce2874cef213fc,
                0xc7489756fd8113bb,
                0xb09f441361ccba15,
                0x396eaa5d408cf7c0,
                0xb0549a62b72c843f,
                0x1201e9d6f45a1,
            ])),
            Fq::from_repr(BigInteger768([
                0x6b5dcd0342cb246c,
                0x6a09c0652a930527,
                0x52beee975dbcf334,
                0x1482fb6099dabbff,
                0xaf7eddad9b94d175,
                0xc9e04d335418f722,
                0xf8898c485e3d9b62,
                0x28a9fc8bc11d7856,
                0x970e355586c80574,
                0x522a2ba8b915b44d,
                0x13f25db5c37781cb,
                0x1310d7455f527,
            ])),
        ),
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0xb6c5438c398dc5b5,
                0x765fe8bc45be7fd3,
                0x1888f1b043abd0a1,
                0x6bf47d2a4b3392bc,
                0xec8fe6ef04f43e2a,
                0x9e563d0212d9a354,
                0xdfa676efdde075fa,
                0xe988084473414e23,
                0x150aecfcf5bca982,
                0xb8774a87351a6201,
                0x8cc7d9e981ec47a1,
                0xa12b0732ee24,
            ])),
            Fq::from_repr(BigInteger768([
                0xaafe130792fe06bb,
                0x700c8597e952d601,
                0xb15f43a7e4d87969,
                0x78f05db77378cc99,
                0x55d396b58085226c,
                0x5298e867ba29a1ed,
                0xc9edc458f424011a,
                0xfedc50815794d7ed,
                0x42797a89a59438b8,
                0x37f39ada55c7e491,
                0x2a39ff4d684b01bc,
                0xfb7c17dfb44d,
            ])),
        ),
    );

    p.add_assign(&G2Projective::new(
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0x45fb0732a6c84ada,
                0x2e7c2e9ea5ec6921,
                0x5f1f798bce840d83,
                0x97624337427f53e7,
                0x68f307a776fa7bb3,
                0x61cdfcd212665871,
                0x21943d55f30ca2ed,
                0x43fa4af2b8e9c1df,
                0xd53bbe66937b8340,
                0x970cf1de31d22d6a,
                0xed583dfe60140adf,
                0x1a81651f3172d,
            ])),
            Fq::from_repr(BigInteger768([
                0x6bbeef6b976ce3d0,
                0xe2beddce175ce60e,
                0x24ba4828635b47ff,
                0x5fbae617c3d3a41a,
                0xf43e46048c3362c,
                0xdc9212692ae366fd,
                0x7425689634949157,
                0xb0350d06d8eb14,
                0x26f6162693ee53f0,
                0x247ffc8a08d0326f,
                0x442163eed14df0b6,
                0x1596b9e069e1b,
            ])),
        ),
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0x8bf03470c8761245,
                0xfd4c6a213c2e80ee,
                0x3eac3a8da5ce68a8,
                0xf85b55e6d1028607,
                0xf70934fbe031ea56,
                0x90acf0ed9ed61fbf,
                0xd075f31aeba31fa8,
                0x96c8f7b8e4ad20de,
                0xcd407824ffa80456,
                0xdb0f203e63ad4b61,
                0xe2407d2665b7ea7,
                0x162cee712acd3,
            ])),
            Fq::from_repr(BigInteger768([
                0x4a8ff1bedf602009,
                0x19d7078a858fd86e,
                0x7b88a3bb30864a03,
                0x8cf9c0d90bb5cb68,
                0xc8eb7d7daf652a9,
                0x7ef37490f02bae3d,
                0xc045597defd4fafb,
                0xe308eb734ec6c080,
                0x56f7b9443ae5e509,
                0x66ff5b397f7678ca,
                0x507aa8b2df44b17f,
                0x8a7672103f13,
            ])),
        ),
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0x7bdb96501b9b12e9,
                0x9877ce67a2f665be,
                0x785f5fab9a28f03e,
                0x2ccf7053154f270,
                0x4f5ca680d79d29f3,
                0xacfd8bc9a2162519,
                0xb064537370fa10f3,
                0xdf123bdce608fa85,
                0x7678c535ab141901,
                0x71901bd3d399070f,
                0xb131e7f825642b39,
                0x109f0e1d55344,
            ])),
            Fq::from_repr(BigInteger768([
                0xa2feeb9ecf8e0085,
                0x224a188ba1aa12d9,
                0xa943620d53b7c16d,
                0x5a0d2e8bb9df45f,
                0x65573960d81635fb,
                0x2c6dbdbdfe722263,
                0xb491103d1f674f2e,
                0xfb5fd8bb11ee2518,
                0xe8f5eefd11529803,
                0x8917c5743ceb6aea,
                0x72ca88afa56ad808,
                0x174b6340e1393,
            ])),
        ),
    ));

    let p = G2Affine::from(p);

    assert_eq!(
        p,
        G2Affine::new(
            Fq2::new(
                Fq::from_repr(BigInteger768([
                    0x81afa3f2e095a548,
                    0x1d233cbfbc96f4a8,
                    0xacc72730733b0e92,
                    0x80830b5a5a6bcf9e,
                    0x814d391aeec16ef0,
                    0x9c9b9c6f85cdae39,
                    0x4df6b22be49d121a,
                    0x798e1285b4ff1b33,
                    0xd22abf08d04dbd78,
                    0x8c4095bb1095d0ef,
                    0x2361e29860806199,
                    0xb4c036809237,
                ])),
                Fq::from_repr(BigInteger768([
                    0x754d967161dad549,
                    0x503e83d0fb16205c,
                    0xd66b0bd4d2f8db2,
                    0xed04664800e94ad5,
                    0xaac6baadf12e5efd,
                    0x17b479ec52ec9bad,
                    0x3c9cd37b95b13d3d,
                    0x60b83e345b25ac21,
                    0x7da690b79d66995d,
                    0xccdc2b8310f5c481,
                    0xe1cb3674dce5cd88,
                    0x58e7be595a24,
                ])),
            ),
            Fq2::new(
                Fq::from_repr(BigInteger768([
                    0xb7cc519ce14c8548,
                    0x978896cb456af97,
                    0x87e8d77b39ec791c,
                    0xfb1e3b1a85794506,
                    0x397bafff7c713104,
                    0xa3759c57f871b6e5,
                    0xd60343332d7b9a6c,
                    0x76c209f8c4b60fb7,
                    0x750c0c62e37bee2,
                    0x67f988ae7c3b298d,
                    0x4fad31acd3b9c7fb,
                    0x169bfaa14a886,
                ])),
                Fq::from_repr(BigInteger768([
                    0xb0f3874c4be6112b,
                    0x4490e35f292e7a55,
                    0x88a51f802f39f7a8,
                    0xba0637f553c02028,
                    0xb475a8a7fe3471a9,
                    0x3833219058c068bf,
                    0x341445c4e97c313f,
                    0x87369d1b2d55de24,
                    0xa2b1d2c4f5c71864,
                    0xf1cafc8965e5dd80,
                    0xa1847d2918041e2d,
                    0x39aaeaabf80f,
                ])),
            ),
            false,
        )
    );
}

#[test]
fn test_g2_doubling_correctness() {
    let mut p = G2Projective::new(
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0x4f40460199b4834d,
                0x5f13a81656e63624,
                0xc9b5774fef6a5b45,
                0xf69a1d904ecdb9c6,
                0xd475ae04f25c898a,
                0xdd439d0bbac3ee1f,
                0xc674397c800da350,
                0x4a8bd6c9959f5a04,
                0xdc606d932a17dab3,
                0xf529b1c08a8ca04e,
                0x12cbb4f27d2f2dbe,
                0x13aa136103c0d,
            ])),
            Fq::from_repr(BigInteger768([
                0x74b1b24ef7988b3a,
                0xf823faebf3cd5b33,
                0xda3de11b2b871de,
                0x9d21221b5e17c092,
                0xbd5820eb76f3d488,
                0x6d66d6b27c131e7d,
                0x4e4b786c56fd3da9,
                0x47cf2a55976d24bb,
                0x668aa26f2dd3e782,
                0xb0a3a87f483667e,
                0xebbd0b140f04e57e,
                0xe6b4e63d455e,
            ])),
        ),
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0x637ef9ea147a4c03,
                0xa8fc73bcfc582623,
                0xa1c12c0fc93087d,
                0x407d5057d43dc90c,
                0x990d3319259da38f,
                0xc22ed48c3b862e15,
                0xb65dbbe810a78712,
                0x4b16691b2c3eb26b,
                0x5a4f9d16d225c5b5,
                0x2247175b0d774cb3,
                0x3ee630223b2e489a,
                0x9b7361aa7381,
            ])),
            Fq::from_repr(BigInteger768([
                0xd88252aa57e34bdc,
                0x1416281f76b6922a,
                0xf9d41eaddb006404,
                0x35c6a06cd179abb9,
                0xfecdb61450f1c4d7,
                0xe82bdc33c1370bff,
                0xc8e36d1922b03f6,
                0x4265e1e317b13ee8,
                0x6f7addbf20e8747f,
                0x11c2c233802d2a84,
                0x793dddc05dfb999c,
                0x40c8f7776907,
            ])),
        ),
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0x64761e7c2d7698f0,
                0x21464f8f08c310a1,
                0xcd1b5bd49bfc070e,
                0xaee4099e6c3753aa,
                0x85394420ca7fc44a,
                0x66694b77b451834a,
                0xbd3d99a5aa3de6f6,
                0xefa977a4c353b40,
                0x20ec15e36ccc7cb6,
                0x4eca29c075ed9a03,
                0x6db651b7e135ebdc,
                0x98676f95f7cd,
            ])),
            Fq::from_repr(BigInteger768([
                0x1a052cc63d15f94b,
                0x81b3051f30815a82,
                0x6fd5819cf1233c1,
                0xd5288b77c0f9d105,
                0x6d054bc7e4f84516,
                0xf3d516571b0f15b5,
                0x4163c185cbbcaf78,
                0x2b18fd0db4b13133,
                0x432aff4610ea17a2,
                0x723c9c7d37db54c9,
                0xa469b46b4f0ca07d,
                0xc1a82c632ac4,
            ])),
        ),
    );

    p.double_in_place();

    let p = G2Affine::from(p);

    assert_eq!(
        p,
        G2Affine::new(
            Fq2::new(
                Fq::from_repr(BigInteger768([
                    0xfa9d19d05f0fcb30,
                    0x35ef65f448fd74c7,
                    0x452a45cce7143629,
                    0x5eb658ad07f48209,
                    0x4bb52a1b39c75349,
                    0xa6f4b4e3171e3d24,
                    0x88360caed283a31e,
                    0x6161974d011ff77,
                    0x440bf0726c0fc78,
                    0x9ba482526fd1e06b,
                    0x9d091d638a429c28,
                    0xb53974cd6fd5,
                ])),
                Fq::from_repr(BigInteger768([
                    0x828e3600d9e9e99f,
                    0xa4b09f6a31cd95dd,
                    0xf672fa14094c4e6a,
                    0xf2a745418b1e2722,
                    0x7d8965fadd4e72e9,
                    0xbd6f5b375067ce7a,
                    0xb4ad0babc4ae06f6,
                    0xecf2bf5923189248,
                    0xf982e77f2aee625d,
                    0x1a4a4607d1f43a15,
                    0xf06f7b40491a6213,
                    0xe05064ac8aea,
                ])),
            ),
            Fq2::new(
                Fq::from_repr(BigInteger768([
                    0x50a100953e6ddb72,
                    0x7e1424572299d433,
                    0x8ba470def85b95cc,
                    0x8a93418009e30f2f,
                    0x5ad6f1e4231ff0c6,
                    0xbb0321bc2b9c54c3,
                    0xfaa3134fc5a57ac3,
                    0x71deb0d9e8179afc,
                    0xa01189be17e9091f,
                    0xe4098264273ff21e,
                    0x8a4ca544ceb6fdf7,
                    0x4c7276b0c360,
                ])),
                Fq::from_repr(BigInteger768([
                    0x9b4f763e35aa7b1f,
                    0x82abc1fb1a0a4671,
                    0x3fff7a3076437911,
                    0x685007b9ec68ff41,
                    0x419d48a4a4cfc31e,
                    0x17b0c54de6b4c534,
                    0x242b5d772462a3ad,
                    0x47ea3a4b867eb3d3,
                    0xcdebcc18e5135841,
                    0xca5456362461a90e,
                    0xcae0297c67cb3270,
                    0x1b8f568cd253e,
                ])),
            ),
            false,
        )
    );
}

#[test]
fn test_g2_affine_projective_conversion() {

    let a = G2Projective::new(
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0xf2a5dc098a75141e,
                0x23e6010de81b1668,
                0xcf749d3de9dbac71,
                0x198cfe64bd15408b,
                0xd6a461232ae70ec,
                0x37da582333e0dd48,
                0x4692c548619682e1,
                0xdcf86b2373460657,
                0x8a63dfaf59522b0d,
                0x2ecccef78efe391d,
                0x291e1e8e9096bbfa,
                0xd12343ac117a,
            ])),
            Fq::from_repr(BigInteger768([
                0xb4347f94cead299c,
                0x62c1a0d3a10e614,
                0x598e1b728540aa39,
                0x2a9e5b0071b768b9,
                0x7781c132d4925830,
                0x62fa7a31076423b9,
                0x2cda45f2c7c17478,
                0x64e950040cdd1e3b,
                0x504d2a8f249d1730,
                0xdd8aa3964168cd9e,
                0x27f7ce59797fb9b9,
                0x5e829c4ba10d,
            ])),
        ),
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0x28ff71b89ad333bd,
                0x32f298aaf5c75f05,
                0xc0be59fcccc3ec6,
                0x68a5f1cfb7784d73,
                0x60ce7df7d47d7b18,
                0x708684767f52e7c8,
                0x511555c106b2dad5,
                0x44508a6d07b9d2c2,
                0x69f4a1a65236af9c,
                0xef20df852b68b2f3,
                0x85e6da13301e6537,
                0x9cefa79553e7,
            ])),
            Fq::from_repr(BigInteger768([
                0x5ecb638625bf6572,
                0xd6ebe33e03362964,
                0xcdff1885b5fe7622,
                0x3bae49ce59768add,
                0xcd48445976684e71,
                0x1984ab71f16c3ebb,
                0x5dcdbd43ad37c101,
                0x2b82d68e7010cce4,
                0xde51df0827df1efb,
                0x94dbc6a585b24210,
                0xab4a0d495b2cbe85,
                0x54b5d1249ad2,
            ])),
        ),
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0xfcf7b07c5f392ac8,
                0x7e0cac31ef62e976,
                0xb66ba07a1174b4b6,
                0xd819e6cec2ba35c,
                0x8c054919779b1228,
                0x9ca9907c610b2e6c,
                0x69c1a21a59b2b3e0,
                0x18500f2cc25c9b75,
                0x82c005e8f5530076,
                0x9ae7dacaec161449,
                0x283d0dcb795334c7,
                0x2b4d22ee7e0a,
            ])),
            Fq::from_repr(BigInteger768([
                0x154115eeb1f386d9,
                0x1966fe1637cca523,
                0x4835a7b4829f5c6c,
                0xfd91d8850839987,
                0x36aec3c30210af0e,
                0x77854447f9fa701,
                0x52fd8ca1f007cedd,
                0xe1bb226c005c3546,
                0xec1ccf3b5c08e477,
                0xbba94ca96e78ad7f,
                0xbf0d58106a40bebb,
                0x182e1c643a30a,
            ])),
        ),
    );

    let a_a = a.into_affine();
    assert_eq!(a_a, G2Affine::new(
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0xd04d1b2e84cfcb48,
                0xa4b0b52c7f856e71,
                0x99add11528f69820,
                0x4f6bf1a726e1f2d3,
                0x28c510ea93c63174,
                0x5bb4ad702d20871b,
                0xc2c1f4140d92e3d8,
                0xac9942226f48234b,
                0xa2a1c4acdb3a9b4c,
                0x6d4a504ee9c8b817,
                0x9cf836fd234056a9,
                0x1701959f3f8df,
            ])),
            Fq::from_repr(BigInteger768([
                0xc5e4764c1e8d6b74,
                0x2ad093258e415ddf,
                0xda887f4bb02ec9cb,
                0x980eb36f879aecb0,
                0xfa49fc664df35b69,
                0xfe883323415d099c,
                0x82e3ac700ed10768,
                0x53aee549d42c7f6f,
                0xaf89293052476fbd,
                0xf8bd3561759d5edc,
                0x7d2b808b4d7796c5,
                0xfbad1bbe2f75,
            ])),
        ),
        Fq2::new(
            Fq::from_repr(BigInteger768([
                0x82499dc3a36723b,
                0x4e50d135b7f54d4e,
                0xdc5f9d26b1632dba,
                0x9cc203059dadfae6,
                0xbad176ed8ce683,
                0x2f13b62bf9447515,
                0x51a60d8f9878ee38,
                0xd0da6a9c191e9932,
                0x7771fda59ea50910,
                0xebb8ee07d88f2a4c,
                0x30a30d31e8371130,
                0x1bc18c8417eb,
            ])),
            Fq::from_repr(BigInteger768([
                0xd00b7ab48a75abe1,
                0x83ff0033936859cd,
                0xab0f91ffc2138a10,
                0x2316611e3c365b00,
                0xe8a21b050527b787,
                0xd30f4581baa3bea0,
                0x59dbf53e112a427b,
                0xe688fc5ea40e5708,
                0xdfbde864b9df0b91,
                0x98b7d79934e7c524,
                0xdca7e75740c36f0e,
                0x1bc31e18981b,
            ])),
        ),
        false
    ));

    assert_eq!(a_a.into_projective(), a);
}