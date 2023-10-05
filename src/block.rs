use super::*;

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
            hash: vec![0, 32],
            prev_block_hash,
            nonce,
            payload,
        }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Fomatter) -> fmt::Result {
        print!("Things will get better with time {}", hash_val);
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
// }
