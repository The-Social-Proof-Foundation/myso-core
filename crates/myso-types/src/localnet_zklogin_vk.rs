use ark_bn254::{Bn254, Fq, Fq2, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_groth16::{Groth16, PreparedVerifyingKey, VerifyingKey};
use ark_snark::SNARK;
use std::str::FromStr;
use std::sync::OnceLock;

fn fq(value: &str) -> Fq {
    Fq::from_str(value).expect("localnet verification key field")
}

fn g1(point: &[&str; 3]) -> G1Affine {
    G1Projective::new(fq(point[0]), fq(point[1]), fq(point[2])).into()
}

fn g2(point: &[[&str; 2]; 3]) -> G2Affine {
    G2Projective::new(
        Fq2::new(fq(point[0][0]), fq(point[0][1])),
        Fq2::new(fq(point[1][0]), fq(point[1][1])),
        Fq2::new(fq(point[2][0]), fq(point[2][1])),
    )
    .into()
}

/// Public Groth16 key exported from `zklogin_myso_final.zkey`
/// (SHA-256 `0b892f2a26827ba5cf9fb0ad936596f940d04e5efb7c8f87d23566775865fc2e`).
/// Callers reach this only through `ZkLoginEnv::Test`.
pub fn localnet_prepared_vk() -> &'static PreparedVerifyingKey<Bn254> {
    static KEY: OnceLock<PreparedVerifyingKey<Bn254>> = OnceLock::new();
    KEY.get_or_init(|| {
        let alpha = g1(&[
            "3346888333150243648794709355723927761822459065892476056335505716600878323800",
            "926404397237202258729328753413409207591164686257092918506957816922888792593",
            "1",
        ]);
        let beta = g2(&[
            [
                "1866796510416044182583074575766191584542136313823384187856088149569251657121",
                "14849186626802286331092753395559856412612056636862613413297590286335882845735",
            ],
            [
                "19203936420100973874064314145065911977106666858128015525962891664550595143840",
                "5147404592994159780651093049009296871599872893985494869445938149464436272246",
            ],
            ["1", "0"],
        ]);
        let gamma = g2(&[
            [
                "10857046999023057135944570762232829481370756359578518086990519993285655852781",
                "11559732032986387107991004021392285783925812861821192530917403151452391805634",
            ],
            [
                "8495653923123431417604973247489272438418190587263600148770280649306958101930",
                "4082367875863433681332203403145435568316851327593401208105741076214120093531",
            ],
            ["1", "0"],
        ]);
        let delta = g2(&[
            [
                "11568366791698561411959961069846140669071496353149410895999403133638837328828",
                "13045764249579535758274181268046559048611668430491321682297467536499039573145",
            ],
            [
                "15527005087329778594736483861659441471853403532140228661051684991286277389909",
                "14945687200013166884100232197350390720440565986376001817771584965981944134516",
            ],
            ["1", "0"],
        ]);
        let gamma_abc = vec![
            g1(&[
                "10327184479025842451900543539668898132565150459385995228028798618925733557038",
                "7688964321762564052753936086976901302946872046684591216468921447625982862017",
                "1",
            ]),
            g1(&[
                "5699527247483006143456523545338490371347205151559905418331515262789849197044",
                "16354967625174179652964140491915861688575143862377984324660589620237734715313",
                "1",
            ]),
        ];
        PreparedVerifyingKey::from(VerifyingKey {
            alpha_g1: alpha,
            beta_g2: beta,
            gamma_g2: gamma,
            delta_g2: delta,
            gamma_abc_g1: gamma_abc,
        })
    })
}

pub fn verify_localnet_groth16(
    proof: &ark_groth16::Proof<Bn254>,
    public_input: ark_bn254::Fr,
) -> Result<bool, ark_relations::r1cs::SynthesisError> {
    Groth16::<Bn254>::verify_with_processed_vk(localnet_prepared_vk(), &[public_input], proof)
}
