use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::common::relay_config::RelayConfig;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RelayEngine {
    config: RelayConfig,
}

impl RelayEngine {
    // Constructor: Creates a new RelayEngine from a config file path
    pub fn from_config_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let config_content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let config = serde_json::from_str::<RelayConfig>(&config_content)
            .map_err(|e| e.to_string())?;
        Ok(RelayEngine { config })
    }

    // Constructor: Creates a new RelayEngine from a RelayEngineConfig instance
    pub fn from_config(config: RelayConfig) -> Self {
        RelayEngine { config }
    }

    // Starts the relay engine
    pub fn start(&self) {
        // Implement your start logic here
        println!("RelayEngine started with config: {:?}", self.config);
    }

    // Stops the relay engine
    pub fn stop(&self) {
        // Implement your stop logic here
        println!("RelayEngine stopped.");
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_config() {
        let config  = RelayConfig {
            streams: Vec::new()
        };

        let engine = RelayEngine::from_config(config.clone());
        assert_eq!(engine.config.streams, config.streams);
    }

    /*
    #[test]
    fn test_from_config_file() {
        let config_path = "test_config.json";
        let config_data = r#"{ "input_streams": [], "output_streams": [] }"#;

        // Create a test config file
        fs::write(config_path, config_data).unwrap();

        let engine = RelayEngine::from_config_file(config_path).unwrap();
        assert_eq!(engine.config.streams.len(), 0);

        // Clean up the test config file
        fs::remove_file(config_path).unwrap();
    }
    */

    #[test]
    fn test_start_stop() {
        let config = RelayConfig {
            streams: Vec::new()
        };

        let engine = RelayEngine::from_config(config);

        engine.start();
        engine.stop();
    }
}
