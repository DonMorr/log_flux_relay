use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StreamsConfig {
    pub stream_configs: Vec<crate::stream::StreamConfig>
}

impl StreamsConfig{
    pub fn new() -> Self {
        StreamsConfig {
            stream_configs: Vec::new()
        }
    }
}