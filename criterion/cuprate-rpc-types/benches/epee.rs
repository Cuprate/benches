//! This module contains benchmarks for any
//!
//! - non-trivial
//! - manual
//! - common
//!
//! type with a `epee` implementation.
//!
//! Types with the standard `epee` derive implementation are not included.

#![allow(unused_attributes, unused_crate_dependencies)]

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

use cuprate_epee_encoding::{from_bytes, to_bytes};
use cuprate_rpc_types::bin::GetBlocksRequest;

use cuprate_criterion_rpc_types::GROUP;

// Enable all the benchmark functions created in this macro.
criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = epee_from_bytes, epee_to_bytes,
}
criterion_main!(benches);

/// Create [`to_bytes`] and [`from_bytes`] benchmarks for `epee` types.
macro_rules! generate_epee_benchmarks {
    (
        $(
            $t:ty
        ),* $(,)?
    ) => { paste::paste! {
        fn epee_from_bytes(c: &mut Criterion) {
            let mut group = c.benchmark_group(format!("{GROUP} (epee, from_bytes)"));

            // Generate the benchmarking functions.
            $(
                let bytes = to_bytes($t::default()).unwrap();

                // `iter_batched()` is used so the `Default::default()`
                // is not part of the timings.
                group.bench_function(stringify!([<$t:snake>]), |b| {
                    b.iter_batched(
                        || bytes.clone(),
                        |mut bytes| drop(from_bytes::<$t, _>(black_box(&mut bytes)).unwrap()),
                        BatchSize::SmallInput,
                    );
                });
            )*
        }

        fn epee_to_bytes(c: &mut Criterion) {
            let mut group = c.benchmark_group(format!("{GROUP} (epee, to_bytes)"));

            $(
                let t = $t::default();
                group.bench_function(stringify!([<$t:snake>]), |b| {
                    b.iter_batched(
                        || t.clone(),
                        |t| drop(to_bytes(black_box(t)).unwrap()),
                        BatchSize::SmallInput,
                    );
                });
            )*
        }
    }};
}

generate_epee_benchmarks! {
    GetBlocksRequest,
    // GetBlocksResponse // TODO: fix epee impl
}
