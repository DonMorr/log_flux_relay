use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use core::fmt;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

pub mod serial_stream;
pub mod file_stream;
pub mod mqtt_stream;
pub mod terminal_stream;
pub mod udp_stream;

use serial_stream::SerialStreamConfig;
use file_stream::FileStreamConfig;
use mqtt_stream::MqttStreamConfig;
use terminal_stream::TerminalStreamConfig;
use udp_stream::UdpStreamConfig;

use crate::message::Message;

pub const INTERNAL_STREAM_TICK_MS: u64 = 10; //Maximum internal TICK rate is 1000/HZ.

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StreamTypeConfig {
    Serial{config: SerialStreamConfig},
    File{config: FileStreamConfig},
    Mqtt{config: MqttStreamConfig},
    Terminal{config: TerminalStreamConfig},
    Udp{config: UdpStreamConfig},
    None
}

impl fmt::Display for StreamTypeConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamTypeConfig::Serial{..} => write!(f, "Serial"),
            StreamTypeConfig::File{..} => write!(f, "File"),
            StreamTypeConfig::Mqtt{..} => write!(f, "Mqtt"),
            StreamTypeConfig::Terminal{..} => write!(f, "Terminal"),
            StreamTypeConfig::Udp{..} => write!(f, "Udp"),
            StreamTypeConfig::None => write!(f, "None")
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StreamConfig {
    pub uuid: Uuid,
    pub name: String,
    pub input_filter: String,
    pub type_config: StreamTypeConfig,
    pub message_delimiter:String,
    pub output_streams: Vec<Uuid>
}

impl StreamConfig {
    pub fn new(uuid: Uuid, name: String, output_streams: Vec<Uuid>, input_filter: String, config:StreamTypeConfig, message_delimiter:String) -> StreamConfig{
        StreamConfig{
            uuid, name, output_streams, input_filter, type_config: config, message_delimiter: message_delimiter
        }
    }

    pub fn add_output_stream(&mut self, output_stream: Uuid) {
        self.output_streams.push(output_stream);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StreamState {
    Initialised,
    Started,
    Paused,
    Ended
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

    thread_handle: Option<JoinHandle<()>>,
    thread_stop_requsted: Arc<AtomicBool>
}

impl StreamCore{

    pub fn new() -> StreamCore {
        let (tx_int_output, rx_int_output) = mpsc::channel::<Message>();
        let (tx_int_input, rx_int_input) = mpsc::channel::<Message>();
        let (tx_ext, rx_ext) = mpsc::channel::<Message>();

        StreamCore {
            state: StreamState::Initialised,
            external_input_sender: tx_ext,
            external_input_receiver: Some(rx_ext),
            external_output_senders: Some(vec![]),
            internal_output_sender: tx_int_output,
            internal_output_receiver: Some(rx_int_output),
            internal_input_sender: tx_int_input,
            internal_input_receiver: Some(rx_int_input),
            thread_handle: Option::None,
            thread_stop_requsted: Arc::new(AtomicBool::new(false))
        }
    }

    pub fn add_external_output(&mut self, sender: Sender<Message>) -> Result<(), String> {
        if let Some(outputs) = &mut self.external_output_senders {
            outputs.push(sender);
            Ok(())
        } else {
            Err("External output senders not available".to_string())
        }
    }

    pub fn add_external_outputs(&mut self, senders: Vec<Sender<Message>>) -> Result<(), String> {
        if let Some(outputs) = &mut self.external_output_senders {
            outputs.append(&mut senders.clone());
            Ok(())
        } else {
            Err("External output senders not available".to_string())
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

    pub fn start(&mut self) -> Result<(), String> {

        if self.state != StreamState::Initialised {
            return Err(String::from("Stream not in correct state to start"))
        }

        let ext_receiver: Receiver<Message> = self.external_input_receiver.take().ok_or("External input receiver unavailable")?;
        let int_receiver: Receiver<Message> = self.internal_input_receiver.take().ok_or("Internal input receiver unavailable")?;
        let ext_outputs: Vec<Sender<Message>> = self.external_output_senders.take().ok_or("External output senders unavailable")?;
        let int_sender: Sender<Message> = self.internal_output_sender.clone();

        let stop_requested = Arc::clone(&self.thread_stop_requsted);

        self.thread_handle = Some(thread::spawn(move || loop {
            // Handle Message received from other Streams
            while let Ok(msg) = ext_receiver.try_recv() {
                // First we filter the messages
                // Todo

                // Forward the message to the internal, specialised stream
                int_sender.send(msg.clone()).unwrap();
                
                // Next we forward the message to the external Streams.
                for output in ext_outputs.iter() {
                    // Forward the message to the external Streams
                    output.send(msg.clone()).unwrap();
                }
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

            // Has stop been requested?
            if stop_requested.load(Ordering::Relaxed) {
                break;
            }
        }));

        self.state = StreamState::Started;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        self.await_thread_stop()
    }

    fn await_thread_stop(&mut self) -> Result<(), String> {
        self.thread_stop_requsted.store(true, Ordering::Relaxed);

        if let Some(thread_handle) = self.thread_handle.take() {
            thread_handle.join().expect("Failed to join core thread");
            Ok(())
        } else {
            Err("Core thread handle not available".to_string())
        }
    }

}

impl Default for StreamConfig{
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: String::from(""),
            output_streams: vec![],
            input_filter: String::from(""),
            type_config: StreamTypeConfig::None,
            message_delimiter: String::from("\n")
        }
    }
}

pub trait Stream {
    fn start(&mut self) -> Result<(), String>;
    fn stop(&mut self) -> Result<(), String>;
    fn get_config(&self) -> &StreamConfig;
    fn get_status(&self) -> &StreamCore;
    fn get_uuid(&self) -> &Uuid;
    fn add_output(&mut self, sender: Sender<Message>) -> Result<(), String>;
    fn add_outputs(&mut self, senders: Vec<Sender<Message>>) -> Result<(), String>;
    fn await_thread_stop(&mut self) -> Result<(), String>;
}
