//! Benchmarks for [`block`] and [`alt_block`] functions.

#![allow(unused_attributes, unused_crate_dependencies)]

use criterion::{black_box as b, criterion_group, criterion_main, BatchSize, Criterion};
use monero_serai::transaction::Output;

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = output_benches,
}
criterion_main!(benches);

fn output_benches(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("{} (output)", monero_serai_criterion::GROUP));

    let txs = b(monero_serai_criterion::txs());

    for (name, tx) in &txs {
        for (i, output) in tx.prefix().outputs.iter().enumerate() {
            let vec = {
                let mut vec = vec![];
                output.write(&mut vec).unwrap();
                Vec::with_capacity(vec.capacity())
            };

            g.bench_function(format!("write_{name}_{i}"), |c| {
                c.iter_batched(
                    || vec.clone(),
                    |mut v| output.write(&mut v),
                    BatchSize::SmallInput,
                );
            });
        }
    }

    for (name, tx) in &txs {
        let rct = tx.version() == 2;
        for (i, output) in tx.prefix().outputs.iter().enumerate() {
            let bytes = output.serialize();
            g.bench_function(format!("read_{name}_{i}"), |c| {
                c.iter(|| Output::read(rct, &mut bytes.as_slice()));
            });
        }
    }

    for (name, tx) in &txs {
        for (i, output) in tx.prefix().outputs.iter().enumerate() {
            g.bench_function(format!("serialize_{name}_{i}"), |c| {
                c.iter_with_large_drop(|| b(output.serialize()));
            });
        }
    }
}
