use serde::{Deserialize, Serialize};
use super::{Stream, StreamConfig};



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FileStreamConfig {
    pub file_name: String,
}