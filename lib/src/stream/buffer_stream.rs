
use std::sync::mpsc::{Sender, Receiver};
use serde::{Deserialize, Serialize};
use super::{Stream, StreamConfig, StreamTypeConfig, Message, StreamCore};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BufferStreamConfig {
    // todo
}


// pub struct BufferStream{

// }

// impl Stream for BufferStream{
//     fn start(&self) -> bool {
//         todo!();
//     }

//     fn stop(&self) -> bool {
//         todo!()
//     }

//     fn pause(&self) -> bool {
//         todo!()
//     }

//     fn get_config(&self) -> &StreamConfig {
//         todo!()
//     }

//     fn get_tx_clone(&self) -> &Sender<Message>{
//         todo!("Implement get_sender");
//     }

//     fn set_receiver(&mut self, receiver: Receiver<Message>){
//         todo!("Implement set_receiver");
//     }

//     fn get_status(&self) -> &StreamCore {
//         todo!("Implement get_status");
//     }
// }

// impl BufferStream {
//     pub fn new(config: StreamConfig) -> Result<Self, &'static str> {
//         if let StreamTypeConfig::Buffer {..} = config.type_config {
//             Ok(Self{})
//         }
//         else{
//             Err("Invalid type_config for a BufferStream")
//         }
//     }
// }