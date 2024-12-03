//! Database operations.
//!
//! This module tests the functions from:
//! - [`cuprate_database::DatabaseRo`]
//! - [`cuprate_database::DatabaseRw`]
//! - [`cuprate_database::DatabaseIter`]

#![allow(unused_crate_dependencies, unused_attributes)]
#![expect(clippy::significant_drop_tightening)]

use std::time::Instant;

use criterion::{
    black_box as b, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup,
    Criterion,
};
use function_name::named;

use cuprate_blockchain::{
    tables::Outputs,
    types::{Output, PreRctOutputId},
};
use cuprate_database::{DatabaseIter, DatabaseRo, DatabaseRw, Env, EnvInner};

use cuprate_criterion_database::{TmpEnv, KEY, VALUE};

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = db_benches
}
criterion_main!(benches);

fn db_benches(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("{} (db)", cuprate_criterion_database::GROUP));

    // `DatabaseRo`
    ro_get(&mut g);
    ro_len(&mut g);
    ro_first(&mut g);
    ro_last(&mut g);
    ro_is_empty(&mut g);
    ro_contains(&mut g);

    // `DatabaseRo` with a `TxRw`
    rw_get(&mut g);
    rw_len(&mut g);
    rw_first(&mut g);
    rw_last(&mut g);
    rw_is_empty(&mut g);
    rw_contains(&mut g);

    // `DatabaseIter`
    get_range(&mut g);
    iter(&mut g);
    keys(&mut g);
    values(&mut g);

    // `DatabaseRw`
    put(&mut g);
    delete(&mut g);
    pop_first(&mut g);
    pop_last(&mut g);
    take(&mut g);
}

//---------------------------------------------------------------------------------------------------- DatabaseRo
// Read-only table operations.
// This uses `TxRw + TablesMut` briefly to insert values, then
// uses `TxRo + Tables` for the actual operation.
//
// See further below for using `TxRw + TablesMut` on the same operations.

/// [`DatabaseRo::get`]
#[named]
fn ro_get(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new().with_key_value();
    let env_inner = env.env.env_inner();
    let tx_ro = env_inner.tx_ro().unwrap();
    let table = env_inner.open_db_ro::<Outputs>(&tx_ro).unwrap();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            let _: Output = table.get(b(&KEY)).unwrap();
        });
    });
}

/// [`DatabaseRo::len`]
#[named]
fn ro_len(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new().with_key_value();
    let env_inner = env.env.env_inner();
    let tx_ro = env_inner.tx_ro().unwrap();
    let table = env_inner.open_db_ro::<Outputs>(&tx_ro).unwrap();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            b(table.len()).unwrap();
        });
    });
}

/// [`DatabaseRo::first`]
#[named]
fn ro_first(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new().with_key_value();
    let env_inner = env.env.env_inner();
    let tx_ro = env_inner.tx_ro().unwrap();
    let table = env_inner.open_db_ro::<Outputs>(&tx_ro).unwrap();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            let (_, _): (PreRctOutputId, Output) = b(table.first()).unwrap();
        });
    });
}

/// [`DatabaseRo::last`]
#[named]
fn ro_last(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new().with_key_value();
    let env_inner = env.env.env_inner();
    let tx_ro = env_inner.tx_ro().unwrap();
    let table = env_inner.open_db_ro::<Outputs>(&tx_ro).unwrap();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            let (_, _): (PreRctOutputId, Output) = b(table.last()).unwrap();
        });
    });
}

/// [`DatabaseRo::is_empty`]
#[named]
fn ro_is_empty(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new().with_key_value();
    let env_inner = env.env.env_inner();
    let tx_ro = env_inner.tx_ro().unwrap();
    let table = env_inner.open_db_ro::<Outputs>(&tx_ro).unwrap();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            b(table.is_empty()).unwrap();
        });
    });
}

/// [`DatabaseRo::contains`]
#[named]
fn ro_contains(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new().with_key_value();
    let env_inner = env.env.env_inner();
    let tx_ro = env_inner.tx_ro().unwrap();
    let table = env_inner.open_db_ro::<Outputs>(&tx_ro).unwrap();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            table.contains(b(&KEY)).unwrap();
        });
    });
}

//---------------------------------------------------------------------------------------------------- DatabaseRo (TxRw)
// These are the same benchmarks as above, but it uses a
// `TxRw` and a `TablesMut` instead to ensure our read/write tables
// using read operations perform the same as normal read-only tables.

/// [`DatabaseRw::get`]
#[named]
fn rw_get(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new().with_key_value();
    let env_inner = env.env.env_inner();
    let tx_rw = env_inner.tx_rw().unwrap();
    let table = env_inner.open_db_rw::<Outputs>(&tx_rw).unwrap();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            let _: Output = table.get(b(&KEY)).unwrap();
        });
    });
}

/// [`DatabaseRw::len`]
#[named]
fn rw_len(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new().with_key_value();
    let env_inner = env.env.env_inner();
    let tx_rw = env_inner.tx_rw().unwrap();
    let table = env_inner.open_db_rw::<Outputs>(&tx_rw).unwrap();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            b(table.len()).unwrap();
        });
    });
}

/// [`DatabaseRw::first`]
#[named]
fn rw_first(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new().with_key_value();
    let env_inner = env.env.env_inner();
    let tx_rw = env_inner.tx_rw().unwrap();
    let table = env_inner.open_db_rw::<Outputs>(&tx_rw).unwrap();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            let (_, _): (PreRctOutputId, Output) = b(table.first()).unwrap();
        });
    });
}

/// [`DatabaseRw::last`]
#[named]
fn rw_last(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new().with_key_value();
    let env_inner = env.env.env_inner();
    let tx_rw = env_inner.tx_rw().unwrap();
    let table = env_inner.open_db_rw::<Outputs>(&tx_rw).unwrap();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            let (_, _): (PreRctOutputId, Output) = b(table.last()).unwrap();
        });
    });
}

/// [`DatabaseRw::is_empty`]
#[named]
fn rw_is_empty(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new().with_key_value();
    let env_inner = env.env.env_inner();
    let tx_rw = env_inner.tx_rw().unwrap();
    let table = env_inner.open_db_rw::<Outputs>(&tx_rw).unwrap();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            b(table.is_empty()).unwrap();
        });
    });
}

/// [`DatabaseRw::contains`]
#[named]
fn rw_contains(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new().with_key_value();
    let env_inner = env.env.env_inner();
    let tx_rw = env_inner.tx_rw().unwrap();
    let table = env_inner.open_db_rw::<Outputs>(&tx_rw).unwrap();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            table.contains(b(&KEY)).unwrap();
        });
    });
}

//---------------------------------------------------------------------------------------------------- DatabaseIter
/// [`DatabaseIter::get_range`]
#[named]
fn get_range(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new().with_key_value_100();
    let env_inner = env.env.env_inner();
    let tx_ro = env_inner.tx_ro().unwrap();
    let table = env_inner.open_db_ro::<Outputs>(&tx_ro).unwrap();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            let range = table.get_range(b(..)).unwrap();
            for result in range {
                let _: Output = b(result.unwrap());
            }
        });
    });
}

/// [`DatabaseIter::iter`]
#[named]
fn iter(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new().with_key_value_100();
    let env_inner = env.env.env_inner();
    let tx_ro = env_inner.tx_ro().unwrap();
    let table = env_inner.open_db_ro::<Outputs>(&tx_ro).unwrap();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            let iter = b(table.iter()).unwrap();
            for result in iter {
                let _: (PreRctOutputId, Output) = b(result.unwrap());
            }
        });
    });
}

/// [`DatabaseIter::keys`]
#[named]
fn keys(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new().with_key_value_100();
    let env_inner = env.env.env_inner();
    let tx_ro = env_inner.tx_ro().unwrap();
    let table = env_inner.open_db_ro::<Outputs>(&tx_ro).unwrap();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            let keys = b(table.keys()).unwrap();
            for result in keys {
                let _: PreRctOutputId = b(result.unwrap());
            }
        });
    });
}

/// [`DatabaseIter::values`]
#[named]
fn values(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new().with_key_value_100();
    let env_inner = env.env.env_inner();
    let tx_ro = env_inner.tx_ro().unwrap();
    let table = env_inner.open_db_ro::<Outputs>(&tx_ro).unwrap();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            let values = b(table.values()).unwrap();
            for result in values {
                let _: Output = b(result.unwrap());
            }
        });
    });
}

//---------------------------------------------------------------------------------------------------- DatabaseRw
/// [`DatabaseRw::put`]
#[named]
fn put(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new();
    let env_inner = env.env.env_inner();
    let tx_rw = env_inner.tx_rw().unwrap();
    let mut table = env_inner.open_db_rw::<Outputs>(&tx_rw).unwrap();

    let mut key = KEY;

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            table.put(b(&key), b(&VALUE)).unwrap();
            key.amount += 1;
        });
    });
}

/// [`DatabaseRw::delete`]
#[named]
fn delete(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new();
    let env_inner = env.env.env_inner();
    let tx_rw = env_inner.tx_rw().unwrap();
    let mut table = env_inner.open_db_rw::<Outputs>(&tx_rw).unwrap();

    let mut key = KEY;

    g.bench_function(function_name!(), |c| {
        c.iter_custom(|iters| {
            for _ in 0..iters {
                table.put(&key, &VALUE).unwrap();
                key.amount += 1;
            }

            key = KEY;

            let start = Instant::now();
            for _ in 0..iters {
                table.delete(&key).unwrap();
                key.amount += 1;
            }
            start.elapsed()
        });
    });
}

/// [`DatabaseRw::pop_first`]
#[named]
fn pop_first(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new();
    let env_inner = env.env.env_inner();
    let tx_rw = env_inner.tx_rw().unwrap();
    let mut table = env_inner.open_db_rw::<Outputs>(&tx_rw).unwrap();

    let mut key = KEY;

    g.bench_function(function_name!(), |c| {
        c.iter_custom(|iters| {
            for _ in 0..iters {
                table.put(&key, &VALUE).unwrap();
                key.amount += 1;
            }

            key = KEY;

            let start = Instant::now();
            for _ in 0..iters {
                table.pop_first().unwrap();
                key.amount += 1;
            }
            start.elapsed()
        });
    });
}

/// [`DatabaseRw::pop_last`]
#[named]
fn pop_last(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new();
    let env_inner = env.env.env_inner();
    let tx_rw = env_inner.tx_rw().unwrap();
    let mut table = env_inner.open_db_rw::<Outputs>(&tx_rw).unwrap();

    let mut key = KEY;

    g.bench_function(function_name!(), |c| {
        c.iter_custom(|iters| {
            for _ in 0..iters {
                table.put(&key, &VALUE).unwrap();
                key.amount += 1;
            }

            key = KEY;

            let start = Instant::now();
            for _ in 0..iters {
                table.pop_last().unwrap();
                key.amount += 1;
            }
            start.elapsed()
        });
    });
}

/// [`DatabaseRw::take`]
#[named]
fn take(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new();
    let env_inner = env.env.env_inner();
    let tx_rw = env_inner.tx_rw().unwrap();
    let mut table = env_inner.open_db_rw::<Outputs>(&tx_rw).unwrap();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            table.put(&KEY, &VALUE).unwrap();
            let _: Output = b(table.take(&b(KEY)).unwrap());
        });
    });
}
