use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub auto_mine: bool,
    pub mine_interval_secs: u64,
    pub data_file: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            auto_mine: true,
            mine_interval_secs: 60,
            data_file: "blockchain.json".into(),
        }
    }
}