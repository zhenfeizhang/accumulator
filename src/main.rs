#![cfg_attr(feature = "cargo-clippy", allow(clippy::cognitive_complexity))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::missing_safety_doc))]
extern crate bigint;
extern crate ff;
extern crate pairing_plus as pairing;
extern crate sha2;

mod accumulate;
mod hash_to_field;
mod param;
mod poly;

use accumulate::*;
use ff::Field;
use pairing::bls12_381::*;
use param::paramgen_from_alpha;
// use poly::*;

fn main() {
    let n = 8;
    let mut two = Fr::one();
    two.add_assign(&Fr::one());
    let pp = paramgen_from_alpha(&two, n);
    let mut values = Vec::with_capacity(n);
    for i in 0..n {
        let s = format!("this is message number {}", i);
        values.push(s.into_bytes());
    }

    let acc = accumulate(&values, &pp);

    let proof = prove(&values, &[0], &pp);

    println!("{}", verify(acc, proof, &[values[0].clone()], &pp));

    println!("Hello, world!");
}
