//! Benchmarks for `cuprate_helper::cast`.
#![allow(unused_attributes, unused_crate_dependencies)]

use criterion::{
    black_box as b, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup,
    Criterion,
};
use function_name::named;

use cuprate_constants::block::MAX_BLOCK_HEIGHT;
use cuprate_helper::map;

use cuprate_criterion_helper::GROUP;

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = map_benches,
}
criterion_main!(benches);

fn map_benches(c: &mut Criterion) {
    let mut g = c.benchmark_group(GROUP);
    combine_low_high_bits_to_u128(&mut g);
    split_u128_into_low_high_bits(&mut g);
    timelock_to_u64(&mut g);
    u64_to_timelock(&mut g);
}

/// Benchmark [`curpate_helper::map::combine_low_high_bits_to_u128`].
#[named]
fn combine_low_high_bits_to_u128(g: &mut BenchmarkGroup<'_, WallTime>) {
    g.bench_function(function_name!(), |bench| {
        bench.iter(|| {
            b(map::combine_low_high_bits_to_u128(b(0), b(0)));
        });
    });
}

/// Benchmark [`curpate_helper::map::split_u128_into_low_high_bits`].
#[named]
fn split_u128_into_low_high_bits(g: &mut BenchmarkGroup<'_, WallTime>) {
    g.bench_function(function_name!(), |bench| {
        bench.iter(|| {
            b(map::split_u128_into_low_high_bits(b(0)));
        });
    });
}

/// Benchmark [`curpate_helper::map::timelock_to_u64`].
#[named]
fn timelock_to_u64(g: &mut BenchmarkGroup<'_, WallTime>) {
    g.bench_function(function_name!(), |bench| {
        bench.iter(|| {
            b(map::timelock_to_u64(b(
                monero_serai::transaction::Timelock::None,
            )));
            b(map::timelock_to_u64(b(
                monero_serai::transaction::Timelock::Time(0),
            )));
            b(map::timelock_to_u64(b(
                monero_serai::transaction::Timelock::Block(0),
            )));
        });
    });
}

/// Benchmark [`curpate_helper::map::u64_to_timelock`].
#[named]
fn u64_to_timelock(g: &mut BenchmarkGroup<'_, WallTime>) {
    g.bench_function(function_name!(), |bench| {
        bench.iter(|| {
            b(map::u64_to_timelock(b(0)));
            b(map::u64_to_timelock(b(MAX_BLOCK_HEIGHT)));
            b(map::u64_to_timelock(b(MAX_BLOCK_HEIGHT + 1)));
        });
    });
}
