//! Benchmarks for [`block`] and [`alt_block`] functions.

#![allow(unused_attributes, unused_crate_dependencies)]

use criterion::{black_box as b, criterion_group, criterion_main, BatchSize, Criterion};
use monero_serai::transaction::Input;

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = input_benches,
}
criterion_main!(benches);

fn input_benches(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("{} (input)", monero_serai_criterion::GROUP));

    let txs = b(monero_serai_criterion::txs());

    for (name, tx) in &txs {
        for (i, input) in tx.prefix().inputs.iter().enumerate() {
            let vec = {
                let mut vec = vec![];
                input.write(&mut vec).unwrap();
                Vec::with_capacity(vec.capacity())
            };

            g.bench_function(format!("write_{name}_{i}"), |c| {
                c.iter_batched(
                    || vec.clone(),
                    |mut v| input.write(&mut v),
                    BatchSize::SmallInput,
                );
            });
        }
    }

    for (name, tx) in &txs {
        for (i, input) in tx.prefix().inputs.iter().enumerate() {
            let bytes = input.serialize();
            g.bench_function(format!("read_{name}_{i}"), |c| {
                c.iter(|| Input::read(&mut bytes.as_slice()));
            });
        }
    }

    for (name, tx) in &txs {
        for (i, input) in tx.prefix().inputs.iter().enumerate() {
            g.bench_function(format!("serialize_{name}_{i}"), |c| {
                c.iter_with_large_drop(|| b(input.serialize()));
            });
        }
    }
}
