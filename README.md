## Benches
This repository contains Cuprate's benchmarks and benchmarking utilities.

See the [`Benchmarking` section in the Architecture book](https://architecture.cuprate.org/benchmarking/intro.html) for documentation.

## Run all benchmarks
```bash
# Run all Criterion benchmarks.
cargo bench

# Run all `cuprate-benchmark` benchmarks.
cargo run --release --package cuprate-benchmark --features all
```

## Historical data
See  <https://benches.hinto.rs> for historical benchmark data.

### Directory structure
```
$MACHINE_NAME/
├── criterion/
│   └── $COMMIT_1..$COMMIT_2/
│       └── $CRITERION_DATA
└── benchmarks/
    └── $COMMIT/
        └── $BENCHMARKING_DATA
```

- The top-level directories are named after the machines being used
- Each top-level directory have 2 sub-directories: `criterion` and `benchmarks`
- Within `criterion`, the data is placed in directories with the `Cuprate/cuprate` commits being compared
- Within `benchmarks`, the data is placed in directories with the `Cuprate/cuprate` commit used

### Machines

| Machine name/directory | CPU                    | Core/thread count | Memory | Disk |
|------------------------|------------------------|-------------------|--------|------|
| moo                    | Intel Xeon E5 2698 v3  | 8/16              | 32 GB  | NVMe SSD