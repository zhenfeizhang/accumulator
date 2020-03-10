#![cfg_attr(feature = "cargo-clippy", allow(clippy::cognitive_complexity))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::missing_safety_doc))]
extern crate bigint;
extern crate ff;
extern crate pairing_plus as pairing;
extern crate sha2;

mod hash_to_field;
mod param;

use ff::Field;
use pairing::bls12_381::*;
use param::paramgen_from_alpha;

fn main() {
    let pp = paramgen_from_alpha(&Fr::one(), 8);

    println!("{:?}", pp);
    println!("Hello, world!");
}
