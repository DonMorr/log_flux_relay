use std::sync::mpsc::Sender;
use uuid::Uuid;
use std::string::String;

use crate::message::Message;
use crate::stream::file_stream::FileStream;
use crate::stream::serial_stream::SerialStream;
use crate::stream::udp_stream::UdpStream;
use crate::stream::waveforms_i2c_stream::WaveformsI2cStream;
use crate::stream::{terminal_stream::TerminalStream, Stream, StreamConfig, StreamTypeConfig};


/// The `StreamsEngine` struct manages a collection of `Stream` instances.
/// It provides methods to add new streams and ensure their UUIDs are unique.
pub struct StreamsEngine{
    streams: Vec<Box<dyn Stream>>
}

/// Manages a collection of `Stream` instances and provides methods to add new streams and ensure their UUIDs are unique.
///
/// The `StreamsEngine` struct is responsible for managing the lifecycle of various stream types, such as terminal, serial, file, and UDP streams. It provides methods to add new streams, link them together, and start/stop the streams. The engine also ensures that the UUIDs of the streams are unique.
///
/// The `add_stream` method is used to add a new stream to the engine, based on the provided `StreamConfig`. The method handles the creation of the appropriate stream type and adds it to the internal `streams` vector.
///
/// The `link_streams` method is responsible for connecting the output streams of each stream to the corresponding input streams, by gathering the UUIDs and senders for each stream's outputs and then adding the collected senders to the corresponding streams.
///
/// The `initialise` method is used to validate the configuration of the streams and then link them together. The `start` method is used to start all the streams managed by the engine.
impl StreamsEngine {
    pub fn new() -> Self {
        StreamsEngine { streams: Vec::new()}
    }

    /// Adds a new stream to the `StreamsEngine` based on the provided `StreamConfig`.
    ///
    /// This method handles the creation of the appropriate stream type (Terminal, Serial, File, or UDP) and adds it to the internal `streams` vector.
    ///
    /// # Arguments
    /// * `config_to_add` - The `StreamConfig` containing the configuration for the new stream to be added.
    ///
    /// # Returns
    /// * `Result<(), String>` - Returns `Ok(())` if the stream was successfully added, or an error message as a `String` if the stream type is invalid.
    pub fn add_stream(&mut self, config_to_add: StreamConfig) -> Result<(), String> {
        println!("Adding stream: {}", config_to_add.name);

        match config_to_add.type_config {
            StreamTypeConfig::Terminal { .. } => {
                let stream = TerminalStream::new(config_to_add)?;
                self.streams.push(Box::new(stream));
            },
            StreamTypeConfig::Serial { .. } => {
                let stream: SerialStream = SerialStream::new(config_to_add)?;
                self.streams.push(Box::new(stream));
            },
            StreamTypeConfig::File { .. } =>  {
                let stream: FileStream = FileStream::new(config_to_add)?;
                self.streams.push(Box::new(stream));
            },
            StreamTypeConfig::Udp { .. } =>  {
                let stream: UdpStream = UdpStream::new(config_to_add)?;
                self.streams.push(Box::new(stream));
            },
            StreamTypeConfig::WaveformsI2c { .. } => {
                let stream: WaveformsI2cStream = WaveformsI2cStream::new(config_to_add)?;
                self.streams.push(Box::new(stream));
            },
            _ => {
                return Err(format!("Invalid stream type: {}", config_to_add.type_config));
            }
        }
        Ok(())
    }

    /// Checks if all the UUIDs in the provided vector are unique.
    ///
    /// This function takes a vector of references to `Uuid` objects and checks if all the UUIDs are unique. It does this by first cloning the input vector, sorting it in place, and then checking for any consecutive duplicate UUIDs.
    ///
    /// # Arguments
    /// * `uuids_orig` - A reference to a vector of references to `Uuid` objects.
    ///
    /// # Returns
    /// * `true` if all the UUIDs in the vector are unique, `false` otherwise.
    fn are_all_uuids_unique(uuids_orig: &Vec<&Uuid>) -> bool {
        let mut uuids = uuids_orig.clone();
        // Sort the vector in place
        uuids.sort();
    
        // Check for consecutive duplicates
        for i in 1..uuids.len() {
            if uuids[i] == uuids[i - 1] {
                return false; // Found a duplicate
            }
        }
        true // No duplicates found
    }

    /// Checks if all the elements in one vector are present in another vector.
    ///
    /// This function takes two vectors of references to `Uuid` objects and checks if all the elements in the first vector are present in the second vector.
    ///
    /// # Arguments
    /// * `vec1` - A reference to a vector of references to `Uuid` objects.
    /// * `vec2` - A reference to a vector of references to `Uuid` objects.
    ///
    /// # Returns
    /// * `true` if all the elements in `vec1` are present in `vec2`, `false` otherwise.
    fn all_elements_in_other(vec1: &Vec<&Uuid>, vec2: &Vec<&Uuid>) -> bool {
        vec1.iter().all(|uuid| vec2.contains(uuid))
    }


    /// Checks if the `StreamsEngine` is in a valid state.
    ///
    /// This function checks the following conditions:
    /// - The UUIDs of the individual streams must be unique.
    /// - The output stream UUIDs of each stream must match the UUIDs of the streams.
    ///
    /// If both conditions are met, the function returns `Ok(())`. Otherwise, it returns an error message.
    fn is_valid(&self) -> Result<(), String> {
        let mut stream_uuids: Vec<&Uuid> = Vec::new();
        let mut output_stream_uuids: Vec<&Uuid> = Vec::new();

        for stream in self.streams.iter() {
            let config: &StreamConfig = stream.get_config();

            stream_uuids.push(&config.uuid);
            for output_stream_uuid in config.output_streams.iter(){
                output_stream_uuids.push(output_stream_uuid);
            }
        }

        // The uuids of the individual streams must be unique.
        if !Self::are_all_uuids_unique(&stream_uuids) {
            return Err("The uuids of the individual streams must be unique.".to_string());
        }

        // The output stream uuids of each stream must match the uuids of the streams.
        if !Self::all_elements_in_other(&output_stream_uuids, &stream_uuids){
            return Err("The output stream uuids of each stream must match the uuids of the streams.".to_string());
        }

        Ok(())
    }

    /// Adds the collected output senders to the corresponding streams in the `StreamsEngine`.
    ///
    /// This function takes a vector of tuples, where each tuple contains a `Uuid` and a vector of `Sender<Message>` objects.
    /// It iterates through the streams in the `StreamsEngine` and, for each stream, it checks if the stream's UUID matches the UUID in the tuple.
    /// If a match is found, the function adds the corresponding vector of `Sender<Message>` objects to the stream.
    ///
    /// # Arguments
    /// * `uuids_with_output_senders` - A vector of tuples, where each tuple contains a `Uuid` and a vector of `Sender<Message>` objects.
    fn add_outputs_to_streams(&mut self, uuids_with_output_senders: Vec<(Uuid, Vec<Sender<Message>>)>) -> Result<(), String> {
        for stream in self.streams.iter_mut() {
            for (uuid, senders) in uuids_with_output_senders.iter() {
                if stream.get_uuid() == uuid {
                    stream.add_outputs(senders.clone())?;
                }
            }
        }
        Ok(())
    }
    
    /// Links the streams in the `StreamsEngine` by gathering the UUIDs and senders for each stream's outputs,
    /// and then adding the collected senders to the corresponding streams.
    ///
    /// This function is divided into two phases:
    /// 1. Gather the UUIDs and senders (no mutable borrow yet)
    /// 2. Mutate the streams (now we do the mutable borrow)
    ///
    /// The first phase iterates through the streams, collects the output senders for each stream, and stores
    /// the UUID and associated senders in a `Vec`. The second phase then calls `add_outputs_to_streams` to
    /// add the collected senders to the corresponding streams.
    fn link_streams(&mut self) -> Result<(), String> {
        // Phase 1: Gather the UUIDs and senders (no mutable borrow yet)
        let mut uuids_with_output_senders: Vec<(Uuid, Vec<Sender<Message>>)> = Vec::new();
    
        for stream in self.streams.iter() {
            let config: &StreamConfig = stream.get_config();
            let mut senders: Vec<Sender<Message>> = Vec::new();
            
            // Gather the output senders for each stream
            for output_uuid in &config.output_streams {
                for stream_inner in self.streams.iter() {
                    if stream_inner.get_uuid() == output_uuid {
                        senders.push(stream_inner.get_status().get_external_input_sender_clone());
                    }
                }
            }
    
            // Store the collected UUID and associated senders
            uuids_with_output_senders.push((stream.get_uuid().clone(), senders));
        }
    
        // Phase 2: Mutate the streams (now we do the mutable borrow)
        self.add_outputs_to_streams(uuids_with_output_senders)?;

        Ok(())
    }

    /// Initializes the `StreamsEngine` by performing the following steps:
    /// 1. Validates the `StreamsEngine` instance using the `is_valid()` method.
    /// 2. Links the streams in the `StreamsEngine` by calling the `link_streams()` method.
    /// 
    /// # Returns
    /// A `Result` with an empty `()` value on success, or a `String` error message on failure.
    pub fn initialise(&mut self) -> Result<(), String> {
        self.is_valid()?;
        self.link_streams()?;
        Ok(())
    }
    
    /// Starts all the streams in the `StreamsEngine`.
    ///
    /// This function iterates through all the streams in the `StreamsEngine` and calls the `start()` method on each one.
    ///
    /// # Returns
    /// A `Result` with an empty `()` value on success, or a `String` error message on failure.
    pub fn start(&mut self) -> Result<(), String> {
        for stream in self.streams.iter_mut() {
            stream.start()?
        }
        Ok(())
    }

    /// Stops all the streams in the `StreamsEngine`.
    ///
    /// This function iterates through all the streams in the `StreamsEngine` and calls the `stop()` method on each one.
    ///
    /// # Returns
    /// A `Result` with an empty `()` value on success, or a `String` error message on failure.
    pub fn stop(&mut self) -> Result<(), String> {
        for stream in self.streams.iter_mut() {
            stream.stop()?
        }
        Ok(())
    }

}
