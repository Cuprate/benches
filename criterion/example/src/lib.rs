#![doc = include_str!("../README.md")] // See the README for crate documentation.
#![allow(unused_crate_dependencies, reason = "used in benchmarks")]

/// All benchmarks performed using this benchmark group will be grouped together in the final report.
pub const GROUP: &str = "example";

/// Shared type that all benchmarks can use.
#[expect(dead_code)]
pub struct SomeHardToCreateObject(u64);

impl From<u64> for SomeHardToCreateObject {
    /// Shared function that all benchmarks can use.
    fn from(value: u64) -> Self {
        Self(value)
    }
}
