//! `cuprate_helper` benchmarks.
#![allow(unused_crate_dependencies)]

mod cryptonight;
mod hashes;
mod randomx;

criterion::criterion_main! {
    hashes::benches,
    cryptonight::benches,
    randomx::benches,
}
