use serde::{Deserialize, Serialize};
use super::stream::{StreamConfig};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RelayConfig {
    pub stream_configs: Vec<StreamConfig>
}