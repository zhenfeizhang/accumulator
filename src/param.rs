use ff::Field;
use pairing::bls12_381::*;
use pairing::{CurveAffine, CurveProjective};

/// Structure for porver parameters.
#[derive(Clone, Debug)]
pub struct Params {
    pub(crate) n: usize,
    pub(crate) g1_gen: Vec<G1Affine>,
    pub(crate) g2_gen: Vec<G2Affine>,
}

/// Internal logic for parameter generation.
/// Will always succeed.
/// Will not be called outside this module.
pub fn paramgen_from_alpha(alpha: &Fr, n: usize) -> Params {
    #[cfg(not(debug_assertions))]
    println!(
        "\n\n\nWarning!!! \nWarning!!! \nWarning!!! \nWarning!!! \n\
        This function (paramgen_from_alpha) shall only be used for developing purpose.\n\
        In deployment you should use `veccom-param` crate to ensure \
        the security of the public parameters.\n\
        End of warning.\n\n"
    );
    let mut g1_gen = Vec::with_capacity(n);
    let mut g2_gen = Vec::with_capacity(n);
    // verifier vector at index i-1 contains g2^{alpha^i} for i ranging from 1 to n
    let mut alpha_power = Fr::one();
    g1_gen.push(G1Affine::one());
    g2_gen.push(G2Affine::one());
    for _ in 0..n {
        alpha_power.mul_assign(&alpha); // compute alpha^i
        g1_gen.push(G1Affine::one().mul(alpha_power).into_affine());
        g2_gen.push(G2Affine::one().mul(alpha_power).into_affine());
    }

    Params { n, g1_gen, g2_gen }
}
