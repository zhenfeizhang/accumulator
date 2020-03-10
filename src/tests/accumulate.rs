use crate::accumulate::*;
use crate::param::*;
use ff::Field;
use pairing::bls12_381::*;
use rand::Rng;
use rand_core::SeedableRng;
#[test]

fn test_accumulator() {
    let mut rng = rand_xorshift::XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);

    let n_array = [8, 16, 32, 64, 128, 512, 1024];
    let ham_array = [1, 2, 4];
    for n in n_array.iter() {
        let alpha = Fr::random(&mut rng);
        let pp = paramgen_from_alpha(&alpha, *n);
        let mut values = Vec::with_capacity(*n);
        for i in 0..*n {
            let s = format!("this is message number {}", i);
            values.push(s.into_bytes());
        }

        let acc = accumulate(&values, &pp);

        for ham in ham_array.iter() {
            println!("testing proofs for n = {} and ham = {}", *n, *ham);
            let index = random_index(*n, *ham);
            let mut proved_values = vec![];
            for e in &index {
                proved_values.push(values[*e].clone());
            }
            let proof = prove(&values, &index, &pp);
            assert!(verify(acc, proof, &proved_values, &pp));
        }
    }
}

fn random_index(n: usize, hamming: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut indices = vec![0u8; n];
    let mut ctr = 0;
    while ctr < hamming {
        let ttry = (rng.gen::<u16>() as usize) % n;
        if indices[ttry] == 0 {
            ctr += 1;
            indices[ttry] = 1;
        }
    }
    let mut res: Vec<usize> = vec![];
    for (i, e) in indices.iter().enumerate().take(n) {
        if *e == 1 {
            res.push(i);
        }
    }
    res
}
