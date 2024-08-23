use serde::{Deserialize, Serialize};
use super::stream::{StreamConfig};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct YalmConfig {
    pub stream_configs: Vec<StreamConfig>
}

impl YalmConfig{
    pub fn new() -> Self {
        YalmConfig {
            stream_configs: Vec::new()
        }
    }
}