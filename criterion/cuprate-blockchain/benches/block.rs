//! Benchmarks for [`block`] and [`alt_block`] functions.

#![allow(unused_attributes, unused_crate_dependencies)]

use std::{num::NonZeroU64, time::Instant};

use criterion::{
    black_box as b, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup,
    Criterion,
};
use cuprate_helper::cast::usize_to_u64;
use function_name::named;

use cuprate_blockchain::{
    cuprate_database::{Env, EnvInner},
    ops::{alt_block, block},
    tables::OpenTables,
};
use cuprate_test_utils::data::{BLOCK_V16_TX0, BLOCK_V1_TX2, BLOCK_V9_TX3};
use cuprate_types::{AltBlockInformation, ChainId, VerifiedBlockInformation};

use cuprate_criterion_blockchain::generate_fake_blocks;

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = block_benches,
}
criterion_main!(benches);

fn block_benches(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("{} (block)", cuprate_criterion_blockchain::GROUP));
    add_block_v1_tx2(&mut g);
    add_block_v9_tx3(&mut g);
    add_block_v16_tx0(&mut g);
    add_alt_block_v1_tx2(&mut g);
    add_alt_block_v9_tx3(&mut g);
    add_alt_block_v16_tx0(&mut g);
}

/// Inner function for benchmarking [`block::add_block`].
#[expect(clippy::significant_drop_tightening)]
fn add_block_inner(
    g: &mut BenchmarkGroup<'_, WallTime>,
    function_name: &str,
    block: &VerifiedBlockInformation,
) {
    let env = cuprate_criterion_blockchain::TmpEnv::new();

    g.bench_function(function_name, |c| {
        // We use `iter_custom` because we need to generate an
        // appropriate amount of blocks and only time the `add_block`.
        c.iter_custom(|count| {
            let blocks = b(generate_fake_blocks(block, count));

            let env_inner = env.env.env_inner();
            let tx_rw = env_inner.tx_rw().unwrap();
            let mut tables = env_inner.open_tables_mut(&tx_rw).unwrap();

            let start = Instant::now();
            for block in &blocks {
                let block = b(block);
                b(block::add_block(block, &mut tables)).unwrap();
            }
            start.elapsed()
        });
    });
}

#[named]
fn add_block_v1_tx2(g: &mut BenchmarkGroup<'_, WallTime>) {
    add_block_inner(g, function_name!(), &BLOCK_V1_TX2);
}

#[named]
fn add_block_v9_tx3(g: &mut BenchmarkGroup<'_, WallTime>) {
    add_block_inner(g, function_name!(), &BLOCK_V9_TX3);
}

#[named]
fn add_block_v16_tx0(g: &mut BenchmarkGroup<'_, WallTime>) {
    add_block_inner(g, function_name!(), &BLOCK_V16_TX0);
}

/// Inner function for benchmarking [`alt_block::add_alt_block`].
#[expect(clippy::significant_drop_tightening)]
fn add_alt_block_inner(
    g: &mut BenchmarkGroup<'_, WallTime>,
    function_name: &str,
    block: &VerifiedBlockInformation,
) {
    let env = cuprate_criterion_blockchain::TmpEnv::new();

    // We must have at least 1 block or else `add_alt_block` will panic.
    {
        let env_inner = env.env.env_inner();
        let tx_rw = env_inner.tx_rw().unwrap();
        let mut tables = env_inner.open_tables_mut(&tx_rw).unwrap();

        let mut block = BLOCK_V1_TX2.clone();
        block.height = 0;

        block::add_block(&block, &mut tables).unwrap();
    }

    g.bench_function(function_name, |c| {
        // We use `iter_custom` because we need to generate an
        // appropriate amount of blocks and only time the `add_block`.
        c.iter_custom(|count| {
            // Map the block to a fake alt block.
            let blocks = generate_fake_blocks(block, count)
                .into_iter()
                .enumerate()
                .map(|(i, b)| AltBlockInformation {
                    block: b.block,
                    block_blob: b.block_blob,
                    txs: b.txs,
                    block_hash: b.block_hash,
                    pow_hash: b.pow_hash,
                    height: b.height + 1,
                    weight: b.weight,
                    long_term_weight: b.long_term_weight,
                    cumulative_difficulty: b.cumulative_difficulty,
                    chain_id: ChainId(NonZeroU64::new(usize_to_u64(i) + 1).unwrap()),
                })
                .collect::<Vec<AltBlockInformation>>();

            let env_inner = env.env.env_inner();
            let tx_rw = env_inner.tx_rw().unwrap();
            let mut tables = env_inner.open_tables_mut(&tx_rw).unwrap();

            let start = Instant::now();
            for block in &blocks {
                let block = b(block);
                b(alt_block::add_alt_block(block, &mut tables)).unwrap();
            }
            start.elapsed()
        });
    });
}

#[named]
fn add_alt_block_v1_tx2(g: &mut BenchmarkGroup<'_, WallTime>) {
    add_alt_block_inner(g, function_name!(), &BLOCK_V1_TX2);
}

#[named]
fn add_alt_block_v9_tx3(g: &mut BenchmarkGroup<'_, WallTime>) {
    add_alt_block_inner(g, function_name!(), &BLOCK_V9_TX3);
}

#[named]
fn add_alt_block_v16_tx0(g: &mut BenchmarkGroup<'_, WallTime>) {
    add_alt_block_inner(g, function_name!(), &BLOCK_V16_TX0);
}
