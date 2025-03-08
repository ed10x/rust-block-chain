use sha2::{Digest, Sha256};
use std::time::SystemTime;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub data: String,
    pub prev_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, data: &str, prev_hash: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Block {
            index,
            timestamp,
            data: data.to_string(),
            prev_hash: prev_hash.to_string(),
            hash: String::new(),
            nonce: 0,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let input = format!("{}{}{}{}", 
            self.index, 
            self.timestamp, 
            self.data, 
            self.prev_hash
        );
        
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }
}