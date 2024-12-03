//! `cuprate_helper` benchmarks.
#![allow(unused_crate_dependencies)]

mod block;
mod block_header;
mod input;
mod output;
mod tx;

criterion::criterion_main! {
    block::benches,
    block_header::benches,
    tx::benches,
    output::benches,
    input::benches,
}
