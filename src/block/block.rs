use crate::block::errors::Result;
use crypto::{digest::Digest, sha2::Sha256};
use log::info;
use std::time::SystemTime;

pub const TARGET_HEXT: usize = 123;
#[derive(Debug, Clone)]
pub struct Block {
    timestamp: u128,
    transactions: Vec<String>,
    prev_block_hash: String,
    hash: String,
    nonce: i32,
    pub height: usize,
}

impl Block {
    pub fn new_block(_data: String, prev_block_hash: String, height: usize) -> Result<Block> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();

        let mut block = Block {
            timestamp,
            transactions: vec!["Hey dude".to_owned()],
            prev_block_hash,
            hash: String::new(),
            height,
            nonce: 0,
        };
        block.run_proof_of_work()?;
        Ok(block)
    }

    pub fn new_genesis_block() -> Block {
        Block::new_block(String::from("Genesis Block"), String::new(), 0).unwrap()
    }

    fn run_proof_of_work(&mut self) -> Result<()> {
        info!("Mining the block");

        while !self.validate()? {
            self.nonce += 1;
        }
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();

        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())
    }

    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        let mut vec1 = vec![];
        vec1.resize(TARGET_HEXT, "0" as u8);
        println!("{:?}", vec1);
        Ok(&hasher.result_str()[0..TARGET_HEXT] == String::from_utf8(vec1))
    }

    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        let content = (
            self.prev_block_hash.clone(),
            self.transactions.clone(),
            self.timestamp,
            TARGET_HEXT,
            self.nonce,
        );
        let bytes = bincode::serialize(&content).unwrap();
        Ok(bytes)
    }

    pub fn get_hash() -> String {
        "".to_owned()
    }
}
