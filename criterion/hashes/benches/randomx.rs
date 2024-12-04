//! Benchmarks for [`Response`].
#![allow(unused_attributes, unused_crate_dependencies, dropping_copy_types)]

use std::{io::Write, time::Duration};

use criterion::{black_box as b, criterion_group, criterion_main, Criterion};

use randomx_rs::{RandomXCache, RandomXDataset, RandomXFlag, RandomXVM};

use hashes_criterion::{GROUP, INPUT};

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(600));
    targets = hash,
}

criterion_main!(benches);

/// Seed used to initialize [`RandomXCache`].
const KEY: &[u8; 32] = &[0; 32];

fn hash(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("{GROUP} (RandomX)"));

    let input_lens = {
        let mut s = String::new();

        for (i, a) in INPUT.iter().enumerate() {
            s += &format!("{}", a.len());

            if i != INPUT.len() {
                s += ", ";
            }
        }

        s
    };

    for (name, vm) in [randomx_vm_default(), randomx_vm_optimized()] {
        for input in INPUT {
            let len = input.len();
            g.bench_function(
                format!("{name} (calculate_hash, input.len() = {len})"),
                |c| {
                    c.iter(|| {
                        b(vm.calculate_hash(b(input))).unwrap();
                    });
                },
            );
        }

        g.bench_function(
            format!("{name} (calculate_hash_set, [input.len(), ...] = [{input_lens}])"),
            |c| {
                c.iter(|| {
                    b(vm.calculate_hash_set(b(INPUT))).unwrap();
                });
            },
        );
    }
}

/// Returns a [`RandomXVM`] with no optimization flags (default, light-verification).
fn randomx_vm_default() -> (&'static str, RandomXVM) {
    const FLAG: RandomXFlag = RandomXFlag::FLAG_DEFAULT;

    let cache = RandomXCache::new(FLAG, KEY).unwrap();
    let vm = RandomXVM::new(FLAG, Some(cache), None).unwrap();

    ("default", vm)
}

/// Returns a [`RandomXVM`] with all optimization flags.
fn randomx_vm_optimized() -> (&'static str, RandomXVM) {
    // TODO: conditional FLAG_LARGE_PAGES, FLAG_JIT

    let mut vm_flag = RandomXFlag::FLAG_HARD_AES | RandomXFlag::FLAG_FULL_MEM;
    let mut cache_flag = RandomXFlag::empty();

    for flag in [&mut vm_flag, &mut cache_flag] {
        match (
            is_x86_feature_detected!("ssse3"),
            is_x86_feature_detected!("avx2"),
        ) {
            (true, _) => *flag |= RandomXFlag::FLAG_ARGON2_SSSE3,
            (_, true) => *flag |= RandomXFlag::FLAG_ARGON2_AVX2,
            (_, _) => *flag |= RandomXFlag::FLAG_ARGON2,
        }
    }

    println!("vm_flag: {vm_flag:#?}");
    println!("cache_flag: {cache_flag:#?}");

    let cache = RandomXCache::new(cache_flag, KEY).unwrap();

    // This takes a while.
    print!("Initializing optimized RandomX dataset... ");
    std::io::stdout().flush().unwrap();
    let dataset = RandomXDataset::new(RandomXFlag::FLAG_DEFAULT, cache, 0).unwrap();
    println!("OK");

    let vm = RandomXVM::new(vm_flag, None, Some(dataset)).unwrap();

    ("optimized", vm)
}
