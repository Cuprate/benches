//! [`Env`] benchmarks.

#![allow(unused_crate_dependencies, unused_attributes)]
#![expect(clippy::significant_drop_tightening)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
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
    targets =
    // open,
    env_inner,
    tx_ro,
    tx_rw,
    open_db_ro,
    open_db_rw,
    create_db,
    resize,
    current_map_size,
    disk_size_bytes,
}
criterion_main!(benches);

fn group() -> String {
    format!("{} (env)", cuprate_criterion_database::GROUP)
}

// FIXME: This function is hard to time due to:
// - heed errors
// - "too many open files" errors
//
// /// [`Env::open`].
// #[named]
// fn open(c: &mut Criterion) {
//     c.bench_function(function_name!(), |b| {
//         b.iter_custom(|_| {
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
fn env_inner(c: &mut Criterion) {
    let mut c = c.benchmark_group(group());
    let env = TmpEnv::new();

    c.bench_function(function_name!(), |b| {
        b.iter(|| {
            drop(black_box(env.env.env_inner()));
        });
    });
}

/// [`EnvInner::tx_ro`].
#[named]
fn tx_ro(c: &mut Criterion) {
    let mut c = c.benchmark_group(group());
    let env = TmpEnv::new();
    let env_inner = env.env.env_inner();

    c.bench_function(function_name!(), |b| {
        b.iter(|| {
            let tx_ro = black_box(env_inner.tx_ro()).unwrap();
            TxRo::commit(black_box(tx_ro)).unwrap();
        });
    });
}

/// [`EnvInner::tx_rw`].
#[named]
fn tx_rw(c: &mut Criterion) {
    let mut c = c.benchmark_group(group());
    let env = TmpEnv::new();
    let env_inner = env.env.env_inner();

    c.bench_function(function_name!(), |b| {
        b.iter(|| {
            let tx_rw = black_box(env_inner.tx_rw()).unwrap();
            TxRw::commit(black_box(tx_rw)).unwrap();
        });
    });
}

/// [`EnvInner::open_db_ro`].
#[named]
fn open_db_ro(c: &mut Criterion) {
    let mut c = c.benchmark_group(group());
    // `with_key_value()` creates the `Outputs`
    // table so the `open_db_ro` below doesn't panic.
    let env = TmpEnv::new().with_key_value();
    let env_inner = env.env.env_inner();
    let tx_ro = env_inner.tx_ro().unwrap();

    c.bench_function(function_name!(), |b| {
        b.iter(|| {
            env_inner.open_db_ro::<Outputs>(&tx_ro).unwrap();
        });
    });
}

/// [`EnvInner::open_db_rw`].
#[named]
fn open_db_rw(c: &mut Criterion) {
    let mut c = c.benchmark_group(group());
    let env = TmpEnv::new();
    let env_inner = env.env.env_inner();
    let tx_rw = env_inner.tx_rw().unwrap();

    c.bench_function(function_name!(), |b| {
        b.iter(|| {
            env_inner.open_db_rw::<Outputs>(&tx_rw).unwrap();
        });
    });
}

/// [`EnvInner::create_db`].
#[named]
fn create_db(c: &mut Criterion) {
    let mut c = c.benchmark_group(group());
    let env = TmpEnv::new();
    let env_inner = env.env.env_inner();
    let tx_rw = env_inner.tx_rw().unwrap();

    c.bench_function(function_name!(), |b| {
        b.iter(|| {
            env_inner.create_db::<Outputs>(&tx_rw).unwrap();
        });
    });
}

/// [`Env::resize`].
#[named]
fn resize(c: &mut Criterion) {
    let mut c = c.benchmark_group(group());
    let env = TmpEnv::new();

    // Resize env.by the OS page size.
    let resize = Some(ResizeAlgorithm::FixedBytes(*PAGE_SIZE));

    c.bench_function(function_name!(), |b| {
        b.iter(|| {
            // This test is only valid for `Env`'s that need to resize manually.
            if ConcreteEnv::MANUAL_RESIZE {
                env.env.resize_map(resize);
            }
        });
    });
}

/// [`Env::current_map_size`].
#[named]
fn current_map_size(c: &mut Criterion) {
    let mut c = c.benchmark_group(group());
    let env = TmpEnv::new();

    c.bench_function(function_name!(), |b| {
        b.iter(|| {
            // This test is only valid for `Env`'s that need to resize manually.
            if ConcreteEnv::MANUAL_RESIZE {
                black_box(env.env.current_map_size());
            }
        });
    });
}

/// [`Env::disk_size_bytes`].
#[named]
fn disk_size_bytes(c: &mut Criterion) {
    let mut c = c.benchmark_group(group());
    let env = TmpEnv::new();

    c.bench_function(function_name!(), |b| {
        b.iter(|| {
            black_box(env.env.disk_size_bytes()).unwrap();
        });
    });
}