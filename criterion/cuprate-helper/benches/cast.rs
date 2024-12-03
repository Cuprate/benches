//! Benchmarks for `cuprate_helper::cast`.
#![allow(unused_attributes, unused_crate_dependencies)]

use criterion::{
    black_box as b, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup,
    Criterion,
};
use function_name::named;

use cuprate_helper::cast::{
    i32_to_isize, i64_to_isize, isize_to_i64, u32_to_usize, u64_to_usize, usize_to_u64,
};

use cuprate_criterion_helper::GROUP;

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = cast_benches,
}
criterion_main!(benches);

fn cast_benches(c: &mut Criterion) {
    let mut g = c.benchmark_group(GROUP);
    integer(&mut g);
    unsigned(&mut g);
}

/// Benchmark integer casts.
#[named]
fn integer(g: &mut BenchmarkGroup<'_, WallTime>) {
    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            b(i32_to_isize(b(0)));
            b(i64_to_isize(b(0)));
            b(isize_to_i64(b(0)));
        });
    });
}

/// Benchmark unsigned integer casts.
#[named]
fn unsigned(g: &mut BenchmarkGroup<'_, WallTime>) {
    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            b(u32_to_usize(b(0)));
            b(u64_to_usize(b(0)));
            b(usize_to_u64(b(0)));
        });
    });
}
