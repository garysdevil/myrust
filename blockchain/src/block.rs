use chrono::prelude::*;
use myutils::cryto;
use serde::{Deserialize, Serialize};

pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String,
    pub pre_hash: String
}

pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String
}
