use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct YalmConfig {
    pub stream_configs: Vec<crate::stream::StreamConfig>
}

impl YalmConfig{
    pub fn new() -> Self {
        YalmConfig {
            stream_configs: Vec::new()
        }
    }
}