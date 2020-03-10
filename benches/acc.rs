#[macro_use]
extern crate criterion;
extern crate accumulator;
extern crate ff;
extern crate pairing_plus as pairing;
extern crate rand;

use accumulator::accumulate::*;
use accumulator::param;
use accumulator::poly;
use criterion::Benchmark;
use criterion::Criterion;
use ff::Field;
use pairing::bls12_381::*;
use rand::Rng;

use rand_core::SeedableRng;
use std::time::Duration;

criterion_group!(acc, bench_acc, poly_eval);
criterion_main!(acc);

fn bench_acc(c: &mut Criterion) {
    let mut rng = rand_xorshift::XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);
    let ham_array = [1, 2, 4, 8, 16, 32];
    let n_array = [64, 128, 256, 512, 1024];
    for n in n_array.iter() {
        let alpha = Fr::random(&mut rng);
        let pp = param::paramgen_from_alpha(&alpha, *n);
        let mut values = Vec::with_capacity(*n);
        for i in 0..*n {
            let s = format!("this is message number {}", i);
            values.push(s.into_bytes());
        }
        let bench_str = format!("bench accumulation, vector size = {}", n);
        let values_clone = values.clone();
        let pp_clone = pp.clone();
        let mut bench = Benchmark::new(&bench_str, move |b| {
            b.iter(|| {
                accumulate(&values_clone, &pp_clone);
            })
        });
        let acc = accumulate(&values, &pp);

        for ham in ham_array.iter() {
            let index = random_index(*n, *ham);
            let mut proved_values = vec![];
            for e in &index {
                proved_values.push(values[*e].clone());
            }

            let bench_str = format!("bench proving, vector size = {}, #proof = {}", n, *ham);
            let values_clone = values.clone();
            let pp_clone = pp.clone();
            let index_clone = index.clone();
            bench = bench.with_function(&bench_str, move |b| {
                b.iter(|| {
                    prove(&values_clone, &index_clone, &pp_clone);
                })
            });

            let bench_str = format!("bench verification, vector size = {}, #proof = {}", n, *ham);
            let proof = prove(&values, &index, &pp);
            let acc_clone = acc.clone();
            let pp_clone = pp.clone();
            bench = bench.with_function(&bench_str, move |b| {
                b.iter(|| {
                    assert!(verify(acc_clone, proof, &proved_values, &pp_clone));
                })
            });
        }

        let bench = bench.warm_up_time(Duration::from_millis(1000));
        let bench = bench.measurement_time(Duration::from_millis(5000));
        let bench = bench.sample_size(100);

        c.bench(&bench_str, bench);
    }
}

fn poly_eval(c: &mut Criterion) {
    let mut rng = rand_xorshift::XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);
    let n_array = [2, 4, 8, 16, 32, 64, 128, 512, 1024];

    for n in n_array.iter() {
        let fr_vec = vec![Fr::random(&mut rng); *n];
        let bench_str = format!("with {} fr elements", *n);

        let bench = Benchmark::new(&bench_str, move |b| {
            b.iter(|| {
                poly::evaluate(&fr_vec);
            })
        });
        let bench = bench.warm_up_time(Duration::from_millis(1000));
        let bench = bench.measurement_time(Duration::from_millis(5000));
        let bench = bench.sample_size(100);

        c.bench(&bench_str, bench);
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
