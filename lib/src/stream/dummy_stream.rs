use serde::{Deserialize, Serialize};
use super::{Stream, StreamConfig, StreamTypeConfig};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DummyStreamConfig{
    pub message_generation_rate_hz: u16
}

impl DummyStreamConfig {
    pub fn new() -> Self {
        DummyStreamConfig {message_generation_rate_hz: 1}
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DummyStream {
    config: StreamConfig
}

impl Stream for DummyStream {
    fn start(&self) {
        todo!("Implement start");
    }

    fn stop(&self) {
        todo!("Implement stop");
    }

    fn pause(&self) {
        todo!("Implement pause");
    }

    fn get_config(&self) -> &StreamConfig {
        &self.config
    }
}

impl DummyStream {
    pub fn new(config: StreamConfig) -> Result<Self, &'static str> {
        if let StreamTypeConfig::Dummy {..} = config.type_config {
            Ok(Self{config})
        }
        else{
            Err("Invalid type_config for a DummyStream")
        }
    }
}
