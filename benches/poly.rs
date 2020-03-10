#[macro_use]
extern crate criterion;
extern crate accumulator;
extern crate ff;
extern crate pairing_plus as pairing;
extern crate rand;

use accumulator::poly;
use criterion::Benchmark;
use criterion::Criterion;
use ff::Field;
use pairing::bls12_381::*;

use rand_core::SeedableRng;
use std::time::Duration;

criterion_group!(poly_bench, poly_eval);
criterion_main!(poly_bench);

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
