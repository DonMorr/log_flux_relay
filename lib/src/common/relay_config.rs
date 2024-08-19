use serde::{Deserialize, Serialize};
use super::stream::Stream;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RelayConfig {
    pub streams: Vec<Stream>
}