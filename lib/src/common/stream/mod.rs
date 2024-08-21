use uuid::Uuid;
use serde::{Deserialize, Serialize};

pub mod serial_stream;

/*
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SocketConfiguration {
    pub ip_address: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FileConfig {
    pub file_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TerminalConfig {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MqttConfig {
    // todo
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BufferConfig {
    // todo
}
*/

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Direction {
    Input,
    Output,
    BiDirectional,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DataType {
    Ascii,
    Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StreamInfo {
    pub uuid: Uuid,
    pub name: String,
    pub direction: Direction,
    pub data_type: DataType,
    pub output_streams: Vec<Uuid>,
    pub input_filter: String
}

impl StreamInfo {
    pub fn new(uuid: Uuid, name: String, direction:Direction, data_type: DataType, output_streams: Vec<Uuid>, input_filter: String) -> StreamInfo{
        StreamInfo{
            uuid, name, direction, data_type, output_streams, input_filter
        }
    }
}

// Define the Stream trait
pub trait Stream {
    fn start(&self);
    fn stop(&self);
    fn pause(&self);
    fn get_info(&self) -> &StreamInfo;
}