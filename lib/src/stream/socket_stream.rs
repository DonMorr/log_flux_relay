use serde::{Deserialize, Serialize};
use super::{Stream, StreamConfig};





#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SocketStreamConfiguration {
    pub ip_address: String,
    pub port: u16,
}