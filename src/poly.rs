use ff::Field;
use pairing::bls12_381::*;

pub fn evaluate(fr_vec: &[Fr]) -> Vec<Fr> {
    let mut res = vec![Fr::one(), fr_vec[0]];
    for e in fr_vec.iter().skip(1) {
        res = poly_mul_by_mono(&res, e);
    }

    assert_eq!(res[0], Fr::one());
    res
}

pub(crate) fn poly_mul_by_mono(poly: &[Fr], fr: &Fr) -> Vec<Fr> {
    let mut res = poly.to_vec();
    let mut tmp = res.clone();
    res.push(Fr::zero());

    for e in &mut tmp {
        e.mul_assign(fr);
    }
    for i in 0..tmp.len() {
        res[i + 1].add_assign(&tmp[i]);
    }
    res
}
