//! Benchmarks for [`block`] and [`alt_block`] functions.

#![allow(unused_attributes, unused_crate_dependencies)]

use criterion::{black_box as b, criterion_group, criterion_main, BatchSize, Criterion};
use monero_serai::block::Block;

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = block_benches,
}
criterion_main!(benches);

fn block_benches(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("{} (block)", monero_serai_criterion::GROUP));

    let blocks = monero_serai_criterion::blocks();

    for (name, block) in &blocks {
        let vec = {
            let mut vec = vec![];
            block.write(&mut vec).unwrap();
            Vec::with_capacity(vec.capacity())
        };

        g.bench_function(format!("write_{name}"), |c| {
            c.iter_batched(
                || vec.clone(),
                |mut v| b(block.write(&mut v)),
                BatchSize::SmallInput,
            );
        });
    }

    for (name, block) in &blocks {
        let bytes = block.serialize();
        g.bench_function(format!("read_{name}"), |c| {
            c.iter_with_large_drop(|| b(Block::read(&mut bytes.as_slice())));
        });
    }

    for (name, block) in &blocks {
        g.bench_function(format!("serialize_{name}"), |c| {
            c.iter_with_large_drop(|| b(block.serialize()));
        });
    }

    for (name, block) in &blocks {
        g.bench_function(format!("serialize_pow_hash_{name}"), |c| {
            c.iter_with_large_drop(|| b(block.serialize_pow_hash()));
        });
    }

    for (name, block) in &blocks {
        g.bench_function(format!("hash_{name}"), |c| {
            c.iter(|| b(block.hash()));
        });
    }
}
