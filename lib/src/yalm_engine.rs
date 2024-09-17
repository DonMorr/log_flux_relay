use std::sync::mpsc::Sender;
use std::{fs, sync::mpsc::Receiver};
use std::path::Path;
use uuid::Uuid;

use crate::stream::serial_stream::SerialStream;
use crate::stream::Message;
use crate::stream::{dummy_stream::DummyStream, Stream, StreamConfig, StreamTypeConfig};
use super::yalm_config::YalmConfig;


pub struct YalmEngine{
    streams: Vec<Box<dyn Stream>>
}

impl YalmEngine {
    pub fn new() -> Self {
        YalmEngine { streams: Vec::new()}
    }

    pub fn add_stream(&mut self, config_to_add: StreamConfig){
        println!("Adding stream: {}", config_to_add.name);

        match config_to_add.type_config {
            // StreamTypeConfig::Serial { .. } => {
            //     let stream = SerialStream::new(config_to_add).unwrap();
            //     self.streams.push(Box::new(stream));
            // },
            // StreamTypeConfig::Buffer { .. } => {
            //     let stream = BufferStream::new(config_to_add).unwrap();
            //     self.streams.push(Box::new(stream));
            // },
            StreamTypeConfig::Dummy { .. } => {
                let stream = DummyStream::new(config_to_add).unwrap();
                self.streams.push(Box::new(stream));
            },
            StreamTypeConfig::Serial { .. } => {
                let stream: SerialStream = SerialStream::new(config_to_add).unwrap();
                self.streams.push(Box::new(stream));
            },
            StreamTypeConfig::Buffer { .. } => todo!(),
            StreamTypeConfig::Socket { config } => todo!(),
            StreamTypeConfig::File { config } => todo!(),
            StreamTypeConfig::Terminal { config } => todo!(),
            StreamTypeConfig::Mqtt { config } => todo!(),
            StreamTypeConfig::None => todo!(),
        }
    }
    
    /*
    // Constructor: Creates a new YalmEngine from a config file path
    pub fn from_config_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let config_content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let config = serde_json::from_str::<YalmConfig>(&config_content)
            .map_err(|e| e.to_string())?;
        Ok(YalmEngine { config })
    }

    // Constructor: Creates a new YalmEngine from a YalmEngineConfig instance
    pub fn from_config(config: YalmConfig) -> Self {
        YalmEngine { config }
    }
    */

    fn all_uuids_unique(uuids_orig: &Vec<&Uuid>) -> bool {
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

    fn all_elements_in_other(vec1: &Vec<&Uuid>, vec2: &Vec<&Uuid>) -> bool {
        vec1.iter().all(|uuid| vec2.contains(uuid))
    }

    fn is_valid(&self) -> bool {
        let mut success: bool = true;
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
        if !Self::all_uuids_unique(&stream_uuids) {
            success = false;
        }

        // The output stream uuids of each stream must match the uuids of the streams.
        if !Self::all_elements_in_other(&output_stream_uuids, &stream_uuids){
            success = false;
        }

        success
    }
    fn add_outputs_to_streams(&mut self, uuids_with_output_senders: Vec<(Uuid, Vec<Sender<Message>>)>) {
        for stream in self.streams.iter_mut() {
            for (uuid, senders) in uuids_with_output_senders.iter() {
                if stream.get_uuid() == uuid {
                    stream.add_outputs(senders.clone());
                }
            }
        }
    }
    
    fn link_streams(&mut self) {
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
        self.add_outputs_to_streams(uuids_with_output_senders);
    }
    
    // fn add_outputs_to_streams(&mut self, uuids_with_output_senders: &Vec<(&Uuid, &Vec<Uuid>, Vec<Sender<Message>>)>){
    //     for stream in self.streams.iter_mut() {
    //         for output_cfg in uuids_with_output_senders.iter() {
    //             if stream.get_uuid() == output_cfg.0 {
    //                 stream.add_outputs(output_cfg.2.clone());
    //             }
    //         }
    //     }
    // }
    
    // fn link_streams(&mut self) {
    //     let mut uuids_with_output_senders: Vec<(&Uuid, &Vec<Uuid>, Vec<Sender<Message>>)> = Vec::new();

    //     for stream in self.streams.iter() {
    //         let config: &StreamConfig = stream.get_config();
    //         let uuid_with_senders: (&Uuid, &Vec<Uuid>, Vec<Sender<Message>>) = (stream.get_uuid(), &config.output_streams, Vec::new());
    //         uuids_with_output_senders.push(uuid_with_senders);
    //     }

    //     for output_cfg in uuids_with_output_senders.iter_mut() {
    //         for output_uuid in output_cfg.1 {
    //             for stream in self.streams.iter() {
    //                 if stream.get_uuid() == output_uuid {
    //                     output_cfg.2.push(stream.get_status().get_external_input_sender_clone());
    //                 }
    //             }
    //         }
    //     }

    //     self.add_outputs_to_streams(&uuids_with_output_senders);
    // }
    
    fn start_streams(&mut self) -> bool {
        let mut success: bool = true;
        for stream in self.streams.iter_mut() {
            if !stream.start() {
                success = false;
            }
        }
        success
    }

    pub fn initialise(&mut self) -> bool {
        let mut success: bool = true;

        if !self.is_valid(){
            success = false;
        }

        self.link_streams();

        success
    }
    
    pub fn start(&mut self) -> bool {
        let mut success: bool = true;

        if !self.start_streams(){
            success = false;
        }

        success
    }

    pub fn stop(&self) {
        println!("YalmEngine stopped.");
    }

}

/*
// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_config() {
        let config  = YalmConfig {
            stream_configs: Vec::new()
        };

        let engine = YalmEngine::from_config(config.clone());
        assert_eq!(engine.config.stream_configs, config.stream_configs);
    }

    
    #[test]
    fn test_from_config_file() {
        let config_path = "test_config.json";
        let config_data = r#"{ "stream_configs": [] }"#;

        // Create a test config file
        fs::write(config_path, config_data).unwrap();

        let engine = YalmEngine::from_config_file(config_path).unwrap();
        assert_eq!(engine.config.stream_configs.len(), 0);

        // Clean up the test config file
        fs::remove_file(config_path).unwrap();
    }
    

    #[test]
    fn test_start_stop() {
        let config = YalmConfig {
            stream_configs: Vec::new()
        };

        let engine = YalmEngine::from_config(config);

        engine.start();
        engine.stop();
    }
}
    */