
use serde::{Deserialize, Serialize};
use super::{Stream, StreamConfig, StreamTypeConfig};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BufferStreamConfig {
    // todo
}


pub struct BufferStream{

}

impl Stream for BufferStream{
    fn start(&self) -> bool {
        todo!();
    }

    fn stop(&self) -> bool {
        todo!()
    }

    fn pause(&self) -> bool {
        todo!()
    }

    fn get_config(&self) -> &StreamConfig {
        todo!()
    }
}

impl BufferStream {
    pub fn new(config: StreamConfig) -> Result<Self, &'static str> {
        if let StreamTypeConfig::Buffer {..} = config.type_config {
            Ok(Self{})
        }
        else{
            Err("Invalid type_config for a BufferStream")
        }
    }
}