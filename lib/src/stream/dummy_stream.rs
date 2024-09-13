use std::sync::mpsc::{Sender, Receiver};

use serde::{Deserialize, Serialize};
use super::{Stream, StreamConfig, StreamTypeConfig, Message, StreamStatus};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DummyStreamConfig{
    pub message_generation_rate_hz: u16
}

impl DummyStreamConfig {
    pub fn new() -> Self {
        DummyStreamConfig {message_generation_rate_hz: 1}
    }
}

#[derive(Debug)]
pub struct DummyStream {
    config: StreamConfig,
    status: StreamStatus
}

impl Stream for DummyStream {

    fn start(&self) -> bool {
        todo!("Implement start");
        false
    }

    fn stop(&self) -> bool {
        todo!("Implement stop");
        false
    }

    fn pause(&self) -> bool {
        todo!("Implement pause");
        false
    }

    fn get_config(&self) -> &StreamConfig {
        &self.config
    }

    fn get_status(&self) -> &StreamStatus {
        &self.status
    }
}

impl DummyStream {
    pub fn new(config: StreamConfig) -> Result<Self, &'static str> {
        if let StreamTypeConfig::Dummy {..} = config.type_config {
            Ok(Self{config: config, status: StreamStatus::new()})
        }
        else{
            Err("Invalid type_config for a DummyStream")
        }
    }
}
