use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::Duration;
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

pub const INTERNAL_STREAM_TICK_MS: u64 = 10; //Maximum internal TICK rate is 1000/HZ.

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
    pub type_config: StreamTypeConfig,
    pub message_delimiter:String,
}

impl StreamConfig {
    pub fn new(uuid: Uuid, name: String, direction:Direction, data_type: DataType, output_streams: Vec<Uuid>, input_filter: String, config:StreamTypeConfig, message_delimiter:String) -> StreamConfig{
        StreamConfig{
            uuid, name, direction, data_type, output_streams, input_filter, type_config: config, message_delimiter: message_delimiter
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
pub struct StreamCore{
    pub state: StreamState,

    // Receiving Messages from external Streams
    external_input_sender: Sender<Message>,
    external_input_receiver: Option<Receiver<Message>>,

    // Sending Messages to external Streams.
    external_output_senders: Option<Vec<Sender<Message>>>,

    // Sending externally received Messages to the internal, specialised stream
    internal_output_sender: Sender<Message>,
    internal_output_receiver: Option<Receiver<Message>>,

    // Receiving Messages generated from the internal, specialised stream.
    internal_input_sender: Sender<Message>,
    internal_input_receiver: Option<Receiver<Message>>,

    thread_handle: Option<JoinHandle<()>>
}

impl StreamCore{

    pub fn new() -> StreamCore {
        let (tx_int_output, rx_int_output) = mpsc::channel::<Message>();
        let (tx_int_input, rx_int_input) = mpsc::channel::<Message>();
        let (tx_ext, rx_ext) = mpsc::channel::<Message>();

        StreamCore {
            state: StreamState::Stopped,
            external_input_sender: tx_ext,
            external_input_receiver: Some(rx_ext),
            external_output_senders: Some(vec![]),
            internal_output_sender: tx_int_output,
            internal_output_receiver: Some(rx_int_output),
            internal_input_sender: tx_int_input,
            internal_input_receiver: Some(rx_int_input),
            thread_handle: Option::None,
        }
    }

    pub fn add_external_output(&mut self, sender: Sender<Message>) {
        if let Some(outputs) = &mut self.external_output_senders {
            outputs.push(sender);
        }
    }

    pub fn add_external_outputs(&mut self, senders: Vec<Sender<Message>>) {
        if let Some(outputs) = &mut self.external_output_senders {
            outputs.append(&mut senders.clone());
        }
    }
    
    pub fn get_external_input_sender_clone(&self) -> Sender<Message>{
        self.external_input_sender.clone()
    }

    /**
     * Used by the specialised stream to get a clone of the internal message sender.
     */
    pub fn get_internal_input_sender_clone(&self) -> Sender<Message>{
        self.internal_input_sender.clone()
    }

    pub fn get_internal_output_receiver(&mut self) -> Receiver<Message>{
        self.internal_output_receiver.take().expect("Internal output receiver unavailable")
    }

    pub fn start(&mut self) {
        let ext_receiver: Receiver<Message> = self.external_input_receiver.take().expect("External receiver unavailable");
        let int_receiver: Receiver<Message> = self.internal_input_receiver.take().expect("Internal receiver unavailable");
        let ext_outputs: Vec<Sender<Message>> = self.external_output_senders.take().expect("Outputs unavailable");
        let int_sender: Sender<Message> = self.internal_output_sender.clone();

        self.thread_handle = Some(thread::spawn(move || loop {
            // Handle Message received from other Streams
            while let Ok(msg) = ext_receiver.try_recv() {
                // First we filter the messages
                // Todo
                
                // Next we forward the message to the external Streams.
                for output in ext_outputs.iter() {
                    // Forward the message to the external Streams
                    output.send(msg.clone());
                }

                // Forward the message to the internal, specialised stream
                int_sender.send(msg.clone());
            }
            
            // Handle Messages received from the internal, specialised Stream
            while let Ok(msg) = int_receiver.try_recv() {
                // First we filter the messages
                // Todo
                
                // Next we forward the message to external Streams
                for output in ext_outputs.iter() {
                    output.send(msg.clone()).unwrap();
                }
            }

            thread::sleep(Duration::from_millis(INTERNAL_STREAM_TICK_MS));
        }));

        self.state = StreamState::Running;
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
            type_config: StreamTypeConfig::None,
            message_delimiter: String::from("\n")
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Message {
    pub timestamp_ms: i64, // Number of milliseconds since EPOC.
    pub originator: String,
    pub text: String,
}

impl Message {
    pub fn new(timestamp: i64, originator: String, text: String) -> Message {
        Message {
            timestamp_ms: timestamp,
            originator,
            text,
        }
    }
}

pub trait Stream {
    fn start(&mut self) -> bool;
    fn stop(&mut self) -> bool;
    fn get_config(&self) -> &StreamConfig;
    fn get_status(&self) -> &StreamCore;
    fn get_uuid(&self) -> &Uuid;
    fn add_output(&mut self, sender: Sender<Message>);
    fn add_outputs(&mut self, senders: Vec<Sender<Message>>);
}


#[cfg(test)]
mod tests {
    use super::*;

}