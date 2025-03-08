use crate::core::block::Block;
use std::fs::File;
use std::io::prelude::*;
use crate::config::AppConfig;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub current_difficulty: u32,
    pub config: AppConfig,
}

impl Blockchain {
    pub fn new(config: AppConfig) -> Self {
        let mut chain = Self {
            chain: Vec::new(),
            current_difficulty: 4,
            config,
        };

        if let Err(_) = chain.load_from_file() {
            let genesis = Block::new(0, "Genesis Block", "0");
            chain.chain.push(genesis);
            chain.save_to_file().unwrap();
        }

        chain
    }

    pub fn save_to_file(&self) -> std::io::Result<()> {
        let serialized = serde_json::to_string_pretty(&self.chain)?;
        let mut file = File::create(&self.config.data_file)?;
        file.write_all(serialized.as_bytes())
    }

    pub fn load_from_file(&mut self) -> std::io::Result<()> {
        let mut file = File::open(&self.config.data_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        self.chain = serde_json::from_str(&contents)?;
        Ok(())
    }

    pub fn add_block(&mut self, data: &str) {
        let prev_block = self.chain.last().unwrap();
        let mut new_block = Block::new(
            prev_block.index + 1,
            data,
            &prev_block.hash
        );
    
        use crate::core::pow::run_proof_of_work;
        run_proof_of_work(&mut new_block, self.current_difficulty);
    
        self.chain.push(new_block);
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i-1];

            if current.hash != current.calculate_hash() {
                return false;
            }

            if current.prev_hash != previous.hash {
                return false;
            }
        }
        true
    }
}