use super::hashable::Hashable;
use std::fmt::Formatter;

// pub mod Block {
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: String,
    pub prev_block_hash: String,
    pub nonce: u64,
    pub payload: String,
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: String,
        nonce: u64,
        payload: String,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: String::from("hash value"),
            prev_block_hash,
            nonce,
            payload,
        }
    }
}

trait Debug {
    fn fmt(&self, f: &mut Formatter);
}

impl Debug for Block {
    // fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    fn fmt(&self, f: &mut Formatter) {
        print!("Things will get better with time {}", &self.hash);
        write!(
            f,
            "Block [{}]: {} at: {} with: {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.payload
        );
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        todo!()
    }
}
