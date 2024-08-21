use serde::{Deserialize, Serialize};
use super::{Stream, StreamInfo};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FlowControl {
    None(),
    XonXoff,
    Etc,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SerialConfiguration {
    pub baud_rate: u32,
    pub port_path: String,
    pub start_bits: u8,
    pub stop_bits: u8,
    pub flow_control: FlowControl,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SerialStream {
    info: StreamInfo,
    config: SerialConfiguration
}

impl Stream for SerialStream {
    fn start(&self) {
        println!("Starting SerialStream: {}", self.get_info().name);
    }

    fn stop(&self) {
        println!("Stopping SerialStream: {}", self.get_info().name);
    }

    fn pause(&self) {
        println!("Pausing SerialStream: {}", self.get_info().name);
    }

    fn get_info(&self) -> &StreamInfo {
        &self.info
    }
}

impl SerialStream {
    pub fn new(info: StreamInfo, config: SerialConfiguration) -> Self {
        SerialStream { info, config }
    }
}