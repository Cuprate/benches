//! [`Env`] benchmarks.

#![allow(unused_crate_dependencies, unused_attributes)]
#![expect(clippy::significant_drop_tightening)]

use criterion::{
    black_box as b, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup,
    Criterion,
};
use function_name::named;

use cuprate_blockchain::tables::Outputs;
use cuprate_database::{
    resize::{ResizeAlgorithm, PAGE_SIZE},
    ConcreteEnv, Env, EnvInner, TxRo, TxRw,
};

use cuprate_criterion_database::TmpEnv;

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = env_benches,
}
criterion_main!(benches);

fn env_benches(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("{} (env)", cuprate_criterion_database::GROUP));
    // open(&mut g);
    env_inner(&mut g);
    tx_ro(&mut g);
    tx_rw(&mut g);
    open_db_ro(&mut g);
    open_db_rw(&mut g);
    create_db(&mut g);
    resize(&mut g);
    current_map_size(&mut g);
    disk_size_bytes(&mut g);
}

// FIXME: This function is hard to time due to:
// - heed errors
// - "too many open files" errors
//
// /// [`Env::open`].
// #[named]
// fn open(c: &mut Criterion) {
//     g.bench_function(function_name!(), |c| {
//         c.iter_custom(|_| {
//             let tempdir = tempfile::tempdir().unwrap();
//             let config = ConfigBuilder::new(tempdir.path().to_path_buf().into()).build();
//
//             let now = std::time::Instant::now();
//             ConcreteEnv::open(config).unwrap();
//             let elapsed = now.elapsed();
//
//             tempdir.close().unwrap();
//             elapsed
//         });
//     });
// }

/// [`Env::env_inner`].
#[named]
fn env_inner(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            drop(b(env.env.env_inner()));
        });
    });
}

/// [`EnvInner::tx_ro`].
#[named]
fn tx_ro(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new();
    let env_inner = env.env.env_inner();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            let tx_ro = b(env_inner.tx_ro()).unwrap();
            TxRo::commit(b(tx_ro)).unwrap();
        });
    });
}

/// [`EnvInner::tx_rw`].
#[named]
fn tx_rw(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new();
    let env_inner = env.env.env_inner();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            let tx_rw = b(env_inner.tx_rw()).unwrap();
            TxRw::commit(b(tx_rw)).unwrap();
        });
    });
}

/// [`EnvInner::open_db_ro`].
#[named]
fn open_db_ro(g: &mut BenchmarkGroup<'_, WallTime>) {
    // `with_key_value()` creates the `Outputs`
    // table so the `open_db_ro` below doesn't panic.
    let env = TmpEnv::new().with_key_value();
    let env_inner = env.env.env_inner();
    let tx_ro = env_inner.tx_ro().unwrap();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            env_inner.open_db_ro::<Outputs>(&tx_ro).unwrap();
        });
    });
}

/// [`EnvInner::open_db_rw`].
#[named]
fn open_db_rw(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new();
    let env_inner = env.env.env_inner();
    let tx_rw = env_inner.tx_rw().unwrap();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            env_inner.open_db_rw::<Outputs>(&tx_rw).unwrap();
        });
    });
}

/// [`EnvInner::create_db`].
#[named]
fn create_db(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new();
    let env_inner = env.env.env_inner();
    let tx_rw = env_inner.tx_rw().unwrap();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            env_inner.create_db::<Outputs>(&tx_rw).unwrap();
        });
    });
}

/// [`Env::resize`].
#[named]
fn resize(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new();

    // Resize env.by the OS page size.
    let resize = Some(ResizeAlgorithm::FixedBytes(*PAGE_SIZE));

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            // This test is only valid for `Env`'s that need to resize manually.
            if ConcreteEnv::MANUAL_RESIZE {
                env.env.resize_map(resize);
            }
        });
    });
}

/// [`Env::current_map_size`].
#[named]
fn current_map_size(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            // This test is only valid for `Env`'s that need to resize manually.
            if ConcreteEnv::MANUAL_RESIZE {
                b(env.env.current_map_size());
            }
        });
    });
}

/// [`Env::disk_size_bytes`].
#[named]
fn disk_size_bytes(g: &mut BenchmarkGroup<'_, WallTime>) {
    let env = TmpEnv::new();

    g.bench_function(function_name!(), |c| {
        c.iter(|| {
            b(env.env.disk_size_bytes()).unwrap();
        });
    });
}
