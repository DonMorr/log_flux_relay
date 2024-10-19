/// The `StreamCore` struct is the core implementation of a stream in the system. It handles the
/// receiving and sending of messages between external and internal stream components.
///
/// The `StreamCore` is responsible for:
/// - Managing the state of the stream (Initialised, Started, Paused, Ended)
/// - Receiving messages from external streams and forwarding them to the internal stream
/// - Receiving messages from the internal stream and forwarding them to external streams
/// - Providing access to the internal message senders and receivers for the specialized stream
/// - Starting and stopping the stream's internal processing thread
///
/// The `StreamCore` is designed to be used as the base implementation for various specialized
/// stream types, such as serial, file, MQTT, terminal, UDP, and Waveforms I2C streams.
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
pub mod waveforms_i2c_stream;

use serial_stream::SerialStreamConfig;
use file_stream::FileStreamConfig;
use mqtt_stream::MqttStreamConfig;
use terminal_stream::TerminalStreamConfig;
use udp_stream::UdpStreamConfig;
use waveforms_i2c_stream::WaveformsI2cStreamConfig;

use crate::message::Message;

pub const INTERNAL_STREAM_TICK_MS: u64 = 10; //Maximum internal TICK rate is 1000/HZ.

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
/// The `StreamTypeConfig` enum represents the different types of stream configurations
/// that can be used in the system. Each variant of the enum contains a configuration
/// struct specific to that stream type.
///
/// The available stream types are:
/// - `Serial`: Represents a serial port stream configuration.
/// - `File`: Represents a file stream configuration.
/// - `Mqtt`: Represents an MQTT stream configuration.
/// - `Terminal`: Represents a terminal stream configuration.
/// - `Udp`: Represents a UDP stream configuration.
/// - `WaveformsI2c`: Represents a Waveforms I2C stream configuration.
/// - `None`: Represents no stream configuration.
pub enum StreamTypeConfig {
    Serial{config: SerialStreamConfig},
    File{config: FileStreamConfig},
    Mqtt{config: MqttStreamConfig},
    Terminal{config: TerminalStreamConfig},
    Udp{config: UdpStreamConfig},
    WaveformsI2c{config: WaveformsI2cStreamConfig},
    None
}

/// Implements the `Display` trait for the `StreamTypeConfig` enum, which allows it to be
/// printed as a string representation of the stream type.
///
/// The `fmt` method is implemented to return a string representation of the stream type
/// for each variant of the `StreamTypeConfig` enum.
impl fmt::Display for StreamTypeConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamTypeConfig::Serial{..} => write!(f, "Serial"),
            StreamTypeConfig::File{..} => write!(f, "File"),
            StreamTypeConfig::Mqtt{..} => write!(f, "Mqtt"),
            StreamTypeConfig::Terminal{..} => write!(f, "Terminal"),
            StreamTypeConfig::Udp{..} => write!(f, "Udp"),
            StreamTypeConfig::WaveformsI2c{..} => write!(f, "WaveformsI2c"),
            StreamTypeConfig::None => write!(f, "None")
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
/// The `StreamConfig` struct represents the configuration for a stream in the system.
/// It contains the following fields:
///
/// - `uuid`: A unique identifier for the stream.
/// - `name`: The name of the stream.
/// - `input_filter`: A filter applied to incoming messages.
/// - `type_config`: The type-specific configuration for the stream.
/// - `message_delimiter`: The delimiter used to separate messages.
/// - `output_streams`: A list of UUIDs for output streams that this stream sends messages to.
pub struct StreamConfig {
    pub uuid: Uuid,
    pub name: String,
    pub input_filter: String,
    pub type_config: StreamTypeConfig,
    pub message_delimiter:String,
    pub output_streams: Vec<Uuid>
}

impl StreamConfig {
    /// Constructs a new `StreamConfig` instance with the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `uuid` - A unique identifier for the stream.
    /// * `name` - The name of the stream.
    /// * `output_streams` - A list of UUIDs for output streams that this stream sends messages to.
    /// * `input_filter` - A filter applied to incoming messages.
    /// * `config` - The type-specific configuration for the stream.
    /// * `message_delimiter` - The delimiter used to separate messages.
    ///
    /// # Returns
    ///
    /// A new `StreamConfig` instance with the provided parameters.
    pub fn new(uuid: Uuid, name: String, output_streams: Vec<Uuid>, input_filter: String, config:StreamTypeConfig, message_delimiter:String) -> StreamConfig{
        StreamConfig{
            uuid, name, output_streams, input_filter, type_config: config, message_delimiter: message_delimiter
        }
    }

    /// Adds an output stream UUID to the list of output streams for this stream.
    ///
    /// # Arguments
    ///
    /// * `output_stream` - The UUID of the output stream to add.
    pub fn add_output_stream(&mut self, output_stream: Uuid) {
        self.output_streams.push(output_stream);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
/// The `StreamState` enum represents the different states a stream can be in.
/// 
/// - `Initialised`: The stream has been created but not started.
/// - `Started`: The stream is actively processing messages.
/// - `Paused`: The stream has been temporarily paused.
/// - `Ended`: The stream has finished processing and is no longer active.
pub enum StreamState {
    Initialised,
    Started,
    Paused,
    Ended
}

#[derive(Debug)]
/// The `StreamCore` struct represents the core functionality of a stream in the application.
/// It manages the input and output channels for both external and internal messages,
/// as well as the state of the stream and the thread handling the stream's processing.
///
/// The `StreamCore` struct is responsible for:
/// - Initializing the input and output channels for external and internal messages
/// - Tracking the current state of the stream (Initialised, Started, Paused, Ended)
/// - Providing methods to add external output senders and get clones of the internal input sender
/// - Storing the thread handle and a flag to request the thread to stop
pub struct StreamCore{
    state: StreamState,

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

    /// Creates a new `StreamCore` instance.
    ///
    /// The `StreamCore` struct is responsible for managing the input and output channels for both external and internal messages,
    /// as well as the state of the stream and the thread handling the stream's processing.
    ///
    /// This constructor initializes the various channels and sets the initial state of the `StreamCore` to `Initialised`.
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

    /// Adds a new external output sender to the stream.
    ///
    /// This method allows adding an additional output channel to the stream, which can be used to forward messages
    /// received from other streams or generated internally.
    ///
    /// # Arguments
    /// * `sender` - The `Sender<Message>` instance to add as an external output.
    ///
    /// # Returns
    /// * `Ok(())` if the sender was successfully added.
    /// * `Err(String)` if the external output senders are not available.
    pub fn add_external_output(&mut self, sender: Sender<Message>) -> Result<(), String> {
        if let Some(outputs) = &mut self.external_output_senders {
            outputs.push(sender);
            Ok(())
        } else {
            Err("External output senders not available".to_string())
        }
    }

    /// Adds a new set of external output senders to the stream.
    ///
    /// This method allows adding additional output channels to the stream, which can be used to forward messages
    /// received from other streams or generated internally.
    ///
    /// # Arguments
    /// * `senders` - A vector of `Sender<Message>` instances to add as external outputs.
    ///
    /// # Returns
    /// * `Ok(())` if the senders were successfully added.
    /// * `Err(String)` if the external output senders are not available.
    pub fn add_external_outputs(&mut self, senders: Vec<Sender<Message>>) -> Result<(), String> {
        if let Some(outputs) = &mut self.external_output_senders {
            outputs.append(&mut senders.clone());
            Ok(())
        } else {
            Err("External output senders not available".to_string())
        }
    }
    
    /**
     * Used by the specialised stream to get a clone of the external message sender.
     */
    pub fn get_external_input_sender_clone(&self) -> Sender<Message>{
        self.external_input_sender.clone()
    }

    /**
     * Used by the specialised stream to get a clone of the internal message sender.
     */
    pub fn get_internal_input_sender_clone(&self) -> Sender<Message>{
        self.internal_input_sender.clone()
    }

    /// Gets a clone of the internal output receiver.
    ///
    /// This method is used by the specialized stream to get a reference to the internal output receiver,
    /// which can be used to receive messages from the main stream.
    pub fn get_internal_output_receiver(&mut self) -> Receiver<Message>{
        self.internal_output_receiver.take().expect("Internal output receiver unavailable")
    }

    /// Starts the stream and begins processing messages.
    ///
    /// This method initializes the necessary components for the stream to start processing messages. It sets up the
    /// external and internal input receivers, the external output senders, and the internal output sender. It then
    /// spawns a new thread that continuously checks for incoming messages from the external and internal input
    /// receivers, processes them, and forwards them to the external output senders.
    ///
    /// The method returns `Ok(())` if the stream was successfully started, or an `Err(String)` if the stream was not
    /// in the correct state to start or if any of the necessary components were unavailable.
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

/// The default implementation for `StreamConfig`.
/// 
/// This implementation sets the following default values:
/// - `uuid`: A new version 4 UUID
/// - `name`: An empty string
/// - `output_streams`: An empty vector
/// - `input_filter`: An empty string
/// - `type_config`: `StreamTypeConfig::None`
/// - `message_delimiter`: The newline character `"\n"`
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

/// The `Stream` trait defines the core functionality for a stream of data.
/// 
/// Streams are responsible for managing the lifecycle of a data source, including
/// starting, stopping, and monitoring the status of the stream. Streams can also
/// manage multiple output channels for the data, allowing multiple consumers to
/// receive the same data.
///
/// Implementations of this trait should handle the details of interacting with
/// the underlying data source, such as connecting to a network, reading from
/// a file, or processing a real-time data feed. The trait provides a common
/// interface for managing these data sources.
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
