use crate::hash_to_field::*;
use crate::param::Params;
use crate::poly;
use ff::{Field, PrimeField};
use pairing::bls12_381::*;
use pairing::Engine;
use pairing::{CurveAffine, CurveProjective};

pub fn accumulate<Blob: AsRef<[u8]>>(values: &[Blob], pp: &Params) -> G1 {
    let fr_vec = hash_to_field_vector(values);

    let basis = poly::evaluate(&fr_vec);
    assert_eq!(basis[0], Fr::one());
    let basis_repr: Vec<FrRepr> = basis.iter().map(|x| x.into_repr()).collect();
    let basis_u64: Vec<&[u64; 4]> = basis_repr.iter().map(|x| &x.0).collect();
    G1Affine::sum_of_products(&pp.g1_gen[0..basis.len()], &basis_u64)
}

pub fn prove<Blob: std::fmt::Debug + std::clone::Clone + AsRef<[u8]>>(
    values: &[Blob],
    indices: &[usize],
    pp: &Params,
) -> G1 {
    let mut indices_local = indices.to_vec();
    let mut values_local = values.to_vec();
    indices_local.sort();
    indices_local.reverse();
    for e in &indices_local {
        values_local.remove(*e);
    }

    accumulate(&values_local, &pp)
}

pub fn verify<Blob: AsRef<[u8]>>(acc: G1, proof: G1, values: &[Blob], pp: &Params) -> bool {
    let fr_vec = hash_to_field_vector(values);
    let basis = poly::evaluate(&fr_vec);
    let basis_repr: Vec<FrRepr> = basis.iter().map(|x| x.into_repr()).collect();
    let basis_u64: Vec<&[u64; 4]> = basis_repr.iter().map(|x| &x.0).collect();
    let p = G2Affine::sum_of_products(&pp.g2_gen[0..basis.len()], &basis_u64);
    let mut t = pp.g2_gen[0].into_projective();
    t.mul_assign(basis[1].into_repr());

    assert_eq!(pp.g2_gen[0], G2Affine::one());
    let mut neg_proof = proof;
    neg_proof.negate();

    Bls12::pairing_product(neg_proof, p, acc, G2::one()) == Fq12::one()
}
