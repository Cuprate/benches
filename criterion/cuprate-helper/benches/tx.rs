//! Benchmarks for `cuprate_helper::cast`.
#![allow(unused_attributes, unused_crate_dependencies)]

use criterion::{
    black_box as b, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup,
    Criterion,
};
use function_name::named;

use cuprate_helper::tx;
use cuprate_test_utils::data::{TX_V1_SIG0, TX_V1_SIG2, TX_V2_RCT3};

use cuprate_criterion_helper::GROUP;

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = tx_benches,
}
criterion_main!(benches);

fn tx_benches(c: &mut Criterion) {
    let mut g = c.benchmark_group(GROUP);
    tx_fee(&mut g);
}

/// Benchmark [`curpate_helper::tx::tx_fee`].
#[named]
fn tx_fee(g: &mut BenchmarkGroup<'_, WallTime>) {
    g.bench_function(function_name!(), |bench| {
        bench.iter(|| {
            b(tx::tx_fee(b(&TX_V1_SIG0.tx)));
            b(tx::tx_fee(b(&TX_V1_SIG2.tx)));
            b(tx::tx_fee(b(&TX_V2_RCT3.tx)));
        });
    });
}
