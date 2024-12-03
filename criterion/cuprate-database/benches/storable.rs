//! [`Storable`] benchmarks.

#![allow(unused_crate_dependencies, unused_attributes)]

use criterion::{
    black_box as b, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup,
    Criterion,
};
use function_name::named;

use cuprate_blockchain::types::{Output, PreRctOutputId};
use cuprate_database::Storable;

use cuprate_criterion_database::{KEY, VALUE};

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = storable_bench,
}
criterion_main!(benches);

fn storable_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("{} (storable)", cuprate_criterion_database::GROUP));
    pre_rct_output_id_as_bytes(&mut g);
    pre_rct_output_id_from_bytes(&mut g);
    output_as_bytes(&mut g);
    output_from_bytes(&mut g);
}

/// [`PreRctOutputId`] cast as bytes.
#[named]
fn pre_rct_output_id_as_bytes(g: &mut BenchmarkGroup<'_, WallTime>) {
    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            b(Storable::as_bytes(b(&KEY)));
        });
    });
}

/// [`PreRctOutputId`] cast from bytes.
#[named]
fn pre_rct_output_id_from_bytes(g: &mut BenchmarkGroup<'_, WallTime>) {
    let bytes = Storable::as_bytes(&KEY);

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            let _: PreRctOutputId = b(Storable::from_bytes(b(bytes)));
        });
    });
}

/// [`Output`] cast as bytes.
#[named]
fn output_as_bytes(g: &mut BenchmarkGroup<'_, WallTime>) {
    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            b(Storable::as_bytes(b(&VALUE)));
        });
    });
}

/// [`Output`] cast from bytes.
#[named]
fn output_from_bytes(g: &mut BenchmarkGroup<'_, WallTime>) {
    let bytes = Storable::as_bytes(&VALUE);

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            let _: Output = b(Storable::from_bytes(b(bytes)));
        });
    });
}
