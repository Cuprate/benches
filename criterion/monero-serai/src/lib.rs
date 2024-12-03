#![allow(unused_crate_dependencies, reason = "used in benchmarks")]

use cuprate_test_utils::data::{
    BLOCK_V16_TX0, BLOCK_V1_TX2, BLOCK_V9_TX3, TX_V1_SIG0, TX_V1_SIG2, TX_V2_RCT3,
};

pub const GROUP: &str = "monero_serai";

pub fn blocks() -> [(&'static str, monero_serai::block::Block); 3] {
    [
        ("block_v1_tx2", BLOCK_V1_TX2.block.clone()),
        ("block_v9_tx3", BLOCK_V9_TX3.block.clone()),
        ("block_v16_tx0", BLOCK_V16_TX0.block.clone()),
    ]
}

pub fn txs() -> [(&'static str, monero_serai::transaction::Transaction); 3] {
    [
        ("tx_v1_sig0", TX_V1_SIG0.tx.clone()),
        ("tx_v1_sig2", TX_V1_SIG2.tx.clone()),
        ("tx_v2_rct3", TX_V2_RCT3.tx.clone()),
    ]
}
