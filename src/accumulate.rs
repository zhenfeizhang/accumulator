use crate::hash_to_field::*;
use crate::param::Params;
use pairing::bls12_381::*;
use pairing::{CurveAffine, CurveProjective};

pub(crate) fn accumulate<Blob: AsRef<[u8]>>(values: &[Blob], pp: &Params) -> G1 {
    let fr_vec = hash_to_field_vector(values);
    G1::zero()
}
