//! Benchmarks for [`Response`].
#![allow(unused_attributes, unused_crate_dependencies, dropping_copy_types)]

use std::time::Duration;

use criterion::{black_box as b, criterion_group, criterion_main, Criterion};

use cuprate_cryptonight::{
    cryptonight_hash_r as r, cryptonight_hash_v0 as v0, cryptonight_hash_v1 as v1,
    cryptonight_hash_v2 as v2,
};
use cuprate_cryptonight_c::{
    cryptonight_hash_r as c_r, cryptonight_hash_v0 as c_v0, cryptonight_hash_v1 as c_v1,
    cryptonight_hash_v2 as c_v2,
};

use hashes_criterion::{GROUP, INPUT};

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(35));
    targets = hash,
}

criterion_main!(benches);

fn hash(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("{GROUP} (CryptoNight)"));

    const R_INPUT: &[(&[u8], u64)] = &[
        (INPUT[0], 500_000),
        (INPUT[1], 500_000),
        (INPUT[2], 500_000),
        (INPUT[3], 500_000),
    ];

    assert_eq!(
        INPUT.len(),
        R_INPUT.len(),
        "update R_INPUT after updating INPUT!",
    );

    for (input, height) in R_INPUT {
        let len = input.len();
        let height = b(*height);

        g.bench_function(format!("r (Rust, input.len() = {len})"), |c| {
            c.iter(|| {
                b(r(b(input), height));
            });
        });
        g.bench_function(format!("r (C, input.len() = {len})"), |c| {
            c.iter(|| {
                b(c_r(b(input), height));
            });
        });
    }

    for input in INPUT {
        let len = input.len();

        g.bench_function(format!("v0 (Rust, input.len() = {len})"), |c| {
            c.iter(|| {
                b(v0(b(input)));
            });
        });
        g.bench_function(format!("v0 (C, input.len() = {len})"), |c| {
            c.iter(|| {
                b(c_v0(b(input)));
            });
        });

        g.bench_function(format!("v1 (Rust, input.len() = {len})"), |c| {
            c.iter(|| {
                b(v1(b(input)));
            });
        });
        g.bench_function(format!("v1 (C, input.len() = {len})"), |c| {
            c.iter(|| {
                b(c_v1(b(input)));
            });
        });

        g.bench_function(format!("v2 (Rust, input.len() = {len})"), |c| {
            c.iter(|| {
                b(v2(b(input)));
            });
        });
        g.bench_function(format!("v2 (C, input.len() = {len})"), |c| {
            c.iter(|| {
                b(c_v2(b(input)));
            });
        });
    }
}
