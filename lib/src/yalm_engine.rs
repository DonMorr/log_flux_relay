use std::fs;
use std::path::Path;
use super::yalm_config::YalmConfig;

pub struct YalmEngine{
    config: YalmConfig
}

impl YalmEngine {
    pub fn new() -> Self {
        YalmEngine { config: YalmConfig::new()}
    }
    
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

    // Starts the relay engine
    pub fn start(&self) {
        // Implement your start logic here
        println!("YalmEngine started with config: {:?}", self.config);
    }

    // Stops the relay engine
    pub fn stop(&self) {
        // Implement your stop logic here
        println!("YalmEngine stopped.");
    }
}

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