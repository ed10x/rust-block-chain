use crate::core::block::Block;
use std::collections::HashMap;

#[derive(Default)]
pub struct MemoryStorage {
    pub blocks: HashMap<u64, Block>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
        }
    }

    pub fn save_block(&mut self, block: &Block) {
        self.blocks.insert(block.index, block.clone());
    }

    pub fn load_blocks(&self) -> Vec<Block> {
        self.blocks.values().cloned().collect()
    }
}