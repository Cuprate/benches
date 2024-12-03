//! Benchmarks for [`block`] and [`alt_block`] functions.

#![allow(unused_attributes, unused_crate_dependencies)]

use criterion::{black_box as b, criterion_group, criterion_main, BatchSize, Criterion};
use monero_serai::transaction::{NotPruned, Transaction};

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = tx_benches,
}
criterion_main!(benches);

fn tx_benches(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("{} (tx)", monero_serai_criterion::GROUP));

    let txs = monero_serai_criterion::txs();

    for (name, tx) in &txs {
        let vec = {
            let mut vec = vec![];
            tx.write(&mut vec).unwrap();
            Vec::with_capacity(vec.capacity())
        };

        g.bench_function(format!("write_{name}"), |c| {
            c.iter_batched(
                || vec.clone(),
                |mut v| b(tx.write(&mut v)),
                BatchSize::SmallInput,
            );
        });
    }

    for (name, tx) in &txs {
        let bytes = tx.serialize();
        g.bench_function(format!("read_{name}"), |c| {
            c.iter_with_large_drop(|| b(Transaction::<NotPruned>::read(&mut bytes.as_slice())));
        });
    }

    for (name, tx) in &txs {
        g.bench_function(format!("serialize_{name}"), |c| {
            c.iter_with_large_drop(|| b(tx.serialize()));
        });
    }

    for (name, tx) in &txs {
        g.bench_function(format!("hash_{name}"), |c| {
            c.iter(|| b(tx.hash()));
        });
    }

    for (name, tx) in &txs {
        g.bench_function(format!("signature_hash_{name}"), |c| {
            c.iter(|| b(tx.signature_hash()));
        });
    }

    for (name, tx) in &txs {
        g.bench_function(format!("weight_{name}"), |c| {
            c.iter(|| b(tx.weight()));
        });
    }
}
