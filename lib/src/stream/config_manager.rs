use serde::{Deserialize, Serialize};
use super::{Stream, StreamConfig};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::io::{Write, Read};

pub fn load_config(config_file_path:String) -> Result<StreamConfig, Box<dyn Error>>{
    let file = File::open(config_file_path)?;
    let reader = BufReader::new(file);
    let config:StreamConfig = serde_json::from_reader(reader)?;
    Ok(config)
}


pub fn save_config(config: &StreamConfig, config_file_path:String) -> Result<(), Box<dyn Error>>{
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(config_file_path)?;
    serde_json::to_writer(&file, &config)?;
    Ok(())
}

/*
#[cfg(test)]
mod tests {
    use crate::stream::{DataType, Direction, StreamTypeConfig};
    use super::*;
    use tempfile::Builder;
    use uuid::Uuid;

    #[test]
    fn test_simple_config_write_and_read(){
     
        let uuid = Uuid::new_v4();
        let name = String::from("Test Stream");
        let direction = Direction::Output;
        let data_type = DataType::Binary;
        let output_streams = vec![Uuid::new_v4(), Uuid::new_v4()];
        let input_filter = String::from("test_filter");
        let config = StreamTypeConfig::None;

        let temp_dir = Builder::new()
        .rand_bytes(5)
        .keep(true)
        .tempdir().unwrap();

        let write_file_path = temp_dir.path().join("test_json_file.json");

        let written_stream_config = StreamConfig::new(
            uuid,
            name.clone(),
            direction,
            data_type,
            output_streams.clone(),
            input_filter.clone(),
            config,
        );
        println!("Using file {}", String::from(write_file_path.to_str().unwrap()));

        save_config(&written_stream_config, String::from(write_file_path.to_str().unwrap()));

        let read_config = load_config(String::from(write_file_path.to_str().unwrap())).unwrap();

        assert_eq!(written_stream_config, read_config);
    }

}*/