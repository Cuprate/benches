//! Benchmarks for [`Response`].
#![allow(unused_attributes, unused_crate_dependencies, dropping_copy_types)]

use criterion::{black_box as b, criterion_group, criterion_main, Criterion};

use hashes_criterion::{GROUP, INPUT};

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = hash,
}

criterion_main!(benches);

fn hash(c: &mut Criterion) {
    let mut g = c.benchmark_group(GROUP);

    for input in INPUT {
        let len = input.len();

        g.bench_function(format!("sha256 (input.len() = {len}"), |c| {
            c.iter(|| {
                use sha2::{Digest, Sha256};
                let _result = b(Sha256::new().chain_update(b(input)).finalize());
            });
        });

        g.bench_function(format!("keccak256 (input.len() = {len}"), |c| {
            c.iter(|| {
                use sha3::{Digest, Keccak256};
                let _result = b(Keccak256::new().chain_update(b(input)).finalize());
            });
        });

        g.bench_function(format!("blake3 (input.len() = {len}"), |c| {
            c.iter(|| {
                let _hash = b(blake3::hash(b(input)));
            });
        });
    }
}
