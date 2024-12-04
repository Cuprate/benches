#![allow(unused_crate_dependencies, reason = "used in benchmarks")]

pub const GROUP: &str = "hashes";

/// Input buffers to hash functions.
pub const INPUT: &[&[u8]] = &[&[3; 8], &[3; 64], &[3; 512], &[3; 65536]];
