use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
pub mod serial_stream;
pub mod buffer_stream;
pub mod file_stream;
pub mod socket_stream;
pub mod mqtt_stream;
pub mod terminal_stream;
pub mod config_manager;
pub mod dummy_stream;

use serial_stream::SerialStreamConfig;
use buffer_stream::BufferStreamConfig;
use file_stream::FileStreamConfig;
use socket_stream::SocketStreamConfiguration;
use mqtt_stream::MqttStreamConfig;
use terminal_stream::TerminalStreamConfig;
use dummy_stream::DummyStreamConfig;


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
pub enum StreamTypeConfig {
    Serial{config: SerialStreamConfig},
    Socket{config: SocketStreamConfiguration},
    File{config: FileStreamConfig},
    Terminal{config: TerminalStreamConfig},
    Mqtt{config: MqttStreamConfig},
    Buffer{config: BufferStreamConfig},
    Dummy{config: DummyStreamConfig},
    None
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StreamConfig {
    pub uuid: Uuid,
    pub name: String,
    pub direction: Direction,
    pub data_type: DataType,
    pub output_streams: Vec<Uuid>,
    pub input_filter: String,
    pub type_config: StreamTypeConfig
}

impl StreamConfig {
    pub fn new(uuid: Uuid, name: String, direction:Direction, data_type: DataType, output_streams: Vec<Uuid>, input_filter: String, config:StreamTypeConfig) -> StreamConfig{
        StreamConfig{
            uuid, name, direction, data_type, output_streams, input_filter, type_config: config
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StreamState {
    Stopped,
    Running,
    Paused
}

#[derive(Debug)]
pub struct StreamStatus{
    pub is_initialized: bool,
    pub state: StreamState,
    sender:Sender<Message>,
    receiver:Receiver<Message>,
    outputs: Vec<Sender<Message>>

}

impl StreamStatus{
    pub fn new() -> StreamStatus{
        let (tx,rx) = mpsc::channel();
        let outputs:Vec<Sender<Message>> = vec![];

        let thread_handle = thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                for output   in &outputs {
                    output.send(msg.clone()).unwrap();
                }
            }
        });

        StreamStatus{
            is_initialized: false, 
            state: StreamState::Stopped,
            sender: tx,
            receiver: rx,
            outputs: outputs
        }
    }
    
    pub fn get_tx_clone(&self) -> Sender<Message>{
        self.sender.clone()
    }

    pub fn add_output_tx(&mut self, receiver: Sender<Message>){
        self.outputs.push(receiver);
    }

    pub fn initialise(&mut self){
        if self.is_initialized {
            todo!("Stream already initialised");
        }

        
        self.is_initialized = true;
    }
}

impl Default for StreamConfig{
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: String::from(""),
            direction: Direction::Input,
            data_type: DataType::Ascii,
            output_streams: vec![],
            input_filter: String::from(""),
            type_config: StreamTypeConfig::None
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Message {
    pub timestamp: u32,
    pub text: String,
}

impl Message {
    pub fn new(timestamp: u32, text: String) -> Message {
        Message {
            timestamp,
            text,
        }
    }
}

pub trait Stream {
    fn start(&self) -> bool;
    fn stop(&self) -> bool;
    fn pause(&self) -> bool;
    fn get_config(&self) -> &StreamConfig;
    fn get_status(&self) -> &StreamStatus;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_config_new() {
        let uuid = Uuid::new_v4();
        let name = String::from("Test Stream");
        let direction = Direction::Output;
        let data_type = DataType::Binary;
        let output_streams = vec![Uuid::new_v4(), Uuid::new_v4()];
        let input_filter = String::from("test_filter");
        let config = StreamTypeConfig::None;

        let stream_config = StreamConfig::new(
            uuid,
            name.clone(),
            direction,
            data_type,
            output_streams.clone(),
            input_filter.clone(),
            config,
        );

        assert_eq!(stream_config.uuid, uuid);
        assert_eq!(stream_config.name, name);
        assert_eq!(stream_config.direction, Direction::Output);
        assert_eq!(stream_config.data_type, DataType::Binary);
        assert_eq!(stream_config.output_streams, output_streams);
        assert_eq!(stream_config.input_filter, input_filter);
        assert!(matches!(stream_config.type_config, StreamTypeConfig::None));
    }

    #[test]
    fn test_stream_config_default() {
        let default_config = StreamConfig::default();

        assert!(!default_config.uuid.is_nil());
        assert_eq!(default_config.name, "");
        assert_eq!(default_config.direction, Direction::Input);
        assert_eq!(default_config.data_type, DataType::Ascii);
        assert!(default_config.output_streams.is_empty());
        assert_eq!(default_config.input_filter, "");
        assert!(matches!(default_config.type_config, StreamTypeConfig::None));
    }

    #[test]
    fn test_stream_config_new_with_empty_values() {
        let uuid = Uuid::nil();
        let name = String::new();
        let direction = Direction::Input;
        let data_type = DataType::Ascii;
        let output_streams = Vec::new();
        let input_filter = String::new();
        let config = StreamTypeConfig::None;

        let stream_config = StreamConfig::new(
            uuid,
            name,
            direction,
            data_type,
            output_streams,
            input_filter,
            config,
        );

        assert_eq!(stream_config.uuid, Uuid::nil());
        assert_eq!(stream_config.name, "");
        assert_eq!(stream_config.direction, Direction::Input);
        assert_eq!(stream_config.data_type, DataType::Ascii);
        assert!(stream_config.output_streams.is_empty());
        assert_eq!(stream_config.input_filter, "");
        assert!(matches!(stream_config.type_config, StreamTypeConfig::None));
    }
}