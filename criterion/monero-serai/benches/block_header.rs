//! Benchmarks for [`block`] and [`alt_block`] functions.

#![allow(unused_attributes, unused_crate_dependencies)]

use criterion::{black_box as b, criterion_group, criterion_main, BatchSize, Criterion};
use monero_serai::block::BlockHeader;

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = block_header_benches,
}
criterion_main!(benches);

fn block_header_benches(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("{} (block_header)", monero_serai_criterion::GROUP));

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
            c.iter_with_large_drop(|| b(BlockHeader::read(&mut bytes.as_slice())));
        });
    }

    for (name, block) in &blocks {
        g.bench_function(format!("serialize_{name}"), |c| {
            c.iter_with_large_drop(|| b(block.serialize()));
        });
    }
}
