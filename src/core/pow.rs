use crate::core::block::Block;
use serde_json::json;
use sha2::{{Digest, Sha256}};

const DIFFICULTY_PREFIX: &str = "0000";

pub fn run_proof_of_work(block: &mut Block, difficulty: u32) {
    let prefix = "0".repeat(difficulty as usize);
    
    loop {
        block.nonce += 1;
        let hash = calculate_hash(block);
        
        if hash.starts_with(&prefix) {
            block.hash = hash;
            break;
        }
    }
}

fn calculate_hash(block: &Block) -> String {
    let serialized = json!({
        "index": block.index,
        "timestamp": block.timestamp,
        "data": block.data,
        "prev_hash": block.prev_hash,
        "nonce": block.nonce
    });
    
    let mut hasher = Sha256::new();
    hasher.update(serialized.to_string().as_bytes());
    format!("{:x}", hasher.finalize())
}