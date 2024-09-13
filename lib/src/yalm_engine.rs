use std::fs;
use std::path::Path;
use uuid::Uuid;

use crate::stream::{buffer_stream::BufferStream, dummy_stream::DummyStream, serial_stream::SerialStream, Stream, StreamConfig, StreamTypeConfig};
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
            StreamTypeConfig::Serial { .. } => {
                let stream = SerialStream::new(config_to_add).unwrap();
                self.streams.push(Box::new(stream));
            },
            StreamTypeConfig::Buffer { .. } => {
                let stream = BufferStream::new(config_to_add).unwrap();
                self.streams.push(Box::new(stream));
            },
            StreamTypeConfig::Dummy { .. } => {
                let stream = DummyStream::new(config_to_add).unwrap();
                self.streams.push(Box::new(stream));
            },
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

        // stream_uuids must be unique
        if !Self::all_uuids_unique(&stream_uuids) {
            success = false;
        }

        // All output_stream_uuids must be in stream_uuids
        if !Self::all_elements_in_other(&output_stream_uuids, &stream_uuids){
            success = false;
        }


        success
    }
    
    fn start_streams(&self) -> bool {
        let mut success: bool = true;
        for stream in self.streams.iter() {
            if !stream.start(){
                success = false;
            }
        }
        success
    }

    // Starts the relay engine
    pub fn start(&self) -> bool {
        let mut success: bool = true;

        if !self.is_valid(){
            success = false;
        }

        if !self.start_streams(){
            success = false;
        }
        
        success
    }

    // Stops the relay engine
    pub fn stop(&self) {
        // Implement your stop logic here
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