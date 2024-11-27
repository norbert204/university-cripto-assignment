use std::time::{SystemTime, UNIX_EPOCH};

use rand::RngCore;

use crate::{helpers::ToHash, transaction};

const POW_ZERO_COUNT: usize = 5;

pub struct BlockChain {
    pub blocks: Vec<Block>,
    pub block_infos: Vec<BlockInfo>,
}

impl BlockChain {
    pub fn new_block(&mut self, transactions: Vec<transaction::Transaction>) {
        let mut rng = rand::thread_rng();

        let previous_merkle_roolt = if self.blocks.len() > 0 {
            Some(self.blocks.last().unwrap().merkle_root.clone())
        } else {
            None
        };

        let previous_hash = if self.blocks.len() > 0 {
            self.blocks.last().unwrap().hash()
        } else {
            String::new()
        };

        let merkle_root = transaction::get_merkle_root(previous_merkle_roolt, &transactions);

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut block = Block {
            nonce: 0,
            merkle_root,
            previous_hash,
            timestamp,
        };

        let mut hash = String::new();

        while !hash.starts_with(&"0".repeat(POW_ZERO_COUNT)) {
            block.nonce = rng.next_u32();

            hash = block.hash();
        }

        self.blocks.push(block);
        self.block_infos.push(BlockInfo { hash, transactions });
    }
}

impl Default for BlockChain {
    fn default() -> Self {
        Self {
            blocks: Vec::new(),
            block_infos: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct BlockInfo {
    pub hash: String,
    pub transactions: Vec<transaction::Transaction>,
}

#[derive(Clone)]
pub struct Block {
    pub previous_hash: String,
    pub nonce: u32,
    pub timestamp: u64,
    pub merkle_root: String,
}

impl ToHash for Block {
    fn hash(&self) -> String {
        let to_hash = format!("{} {} {} {}", self.nonce, self.merkle_root, self.timestamp, self.previous_hash);

        sha256::digest(to_hash)
    }
}
