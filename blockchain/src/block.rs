use chrono::prelude::Utc;
use myutils::util_cryto;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub time: i64,
    pub merkle_root: String,
    pub prev_blockhash: String,
}

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    /// List of transactions contained in the block // 理论上数据类型应该是集合
    pub data: String,
}

impl Block {
    // fn set_hash

    pub fn new_block(data: String, prev_blockhash: String) -> Block {
        let transaction = util_cryto::get_serialize(&data);
        let merkle_root = util_cryto::get_hash(&transaction[..]);

        let time = Utc::now().timestamp();

        let block = Block {
            header: BlockHeader {
                time: time,
                merkle_root: merkle_root,
                prev_blockhash: prev_blockhash,
            },
            data: data,
        };
        block
    }
}
