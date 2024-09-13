use serde::{Deserialize, Serialize};
use super::{Stream, StreamConfig, StreamTypeConfig};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FlowControl {
    None(),
    XonXoff,
    Etc,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SerialStreamConfig {
    pub baud_rate: u32,
    pub port_path: String,
    pub start_bits: u8,
    pub stop_bits: u8,
    pub flow_control: FlowControl,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SerialStream {
    config: StreamConfig
}

impl Stream for SerialStream { 
    fn start(&self) -> bool {
        todo!("Implement start");
    }

    fn stop(&self) -> bool {
        todo!("Implement stop");
    }

    fn pause(&self) -> bool {
        todo!("Implement pause");
    }

    fn get_config(&self) -> &StreamConfig {
        &self.config
    }
}

impl SerialStream {
    pub fn new(config: StreamConfig) -> Result<Self, &'static str> {
        if let StreamTypeConfig::Serial {..} = config.type_config {
            Ok(Self{config})
        }
        else{
            Err("Invalid type_config for a SerialStream")
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::stream::{DataType, Direction};

    use super::*;

    #[test]
    fn test_serial_stream_new_valid_config() {
        let uuid = Uuid::new_v4();
        let name = String::from("Test Serial Stream");
        let direction = Direction::Input;
        let data_type = DataType::Binary;
        let output_streams = vec![];
        let input_filter = String::new();

        let serial_config = SerialStreamConfig {
            baud_rate: 9600,
            port_path: String::from("/dev/ttyUSB0"),
            start_bits: 1,
            stop_bits: 1,
            flow_control: FlowControl::None(),
        };
        let type_config = StreamTypeConfig::Serial{config: serial_config};

        let stream_config = StreamConfig::new(
            uuid,
            name,
            direction,
            data_type,
            output_streams,
            input_filter,
            type_config,
        );

        let result = SerialStream::new(stream_config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_serial_stream_new_invalid_config() {
        let uuid = Uuid::new_v4();
        let name = String::from("Test Invalid Stream");
        let direction = Direction::Input;
        let data_type = DataType::Binary;
        let output_streams = vec![];
        let input_filter = String::new();
        let type_config = StreamTypeConfig::None;

        let stream_config = StreamConfig::new(
            uuid,
            name,
            direction,
            data_type,
            output_streams,
            input_filter,
            type_config,
        );

        let result = SerialStream::new(stream_config);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid type_config for a SerialStream");
    }

    #[test]
    fn test_serial_stream_get_info() {
        let uuid = Uuid::new_v4();
        let name = String::from("Test Serial Stream");
        let direction = Direction::Output;
        let data_type = DataType::Ascii;
        let output_streams = vec![Uuid::new_v4()];
        let input_filter = String::from("test_filter");
        let serial_config = SerialStreamConfig {
            baud_rate: 115200,
            port_path: String::from("/dev/ttyUSB1"),
            start_bits: 1,
            stop_bits: 2,
            flow_control: FlowControl::XonXoff,
        };
        let type_config = StreamTypeConfig::Serial{config: serial_config};

        let stream_config = StreamConfig::new(
            uuid,
            name.clone(),
            direction,
            data_type,
            output_streams.clone(),
            input_filter.clone(),
            type_config,
        );

        let serial_stream = SerialStream::new(stream_config).unwrap();
        let info = serial_stream.get_config();

        assert_eq!(info.uuid, uuid);
        assert_eq!(info.name, name);
        assert_eq!(info.direction, Direction::Output);
        assert_eq!(info.data_type, DataType::Ascii);
        assert_eq!(info.output_streams, output_streams);
        assert_eq!(info.input_filter, input_filter);
        assert!(matches!(info.type_config, StreamTypeConfig::Serial { .. }));
    }

    #[test]
    fn test_flow_control_variants() {
        assert_ne!(FlowControl::None(), FlowControl::XonXoff);
        assert_ne!(FlowControl::None(), FlowControl::Etc);
        assert_ne!(FlowControl::XonXoff, FlowControl::Etc);
    }

    #[test]
    fn test_serial_stream_config_equality() {
        let config1 = SerialStreamConfig {
            baud_rate: 9600,
            port_path: String::from("/dev/ttyUSB0"),
            start_bits: 1,
            stop_bits: 1,
            flow_control: FlowControl::None(),
        };

        let config2 = SerialStreamConfig {
            baud_rate: 9600,
            port_path: String::from("/dev/ttyUSB0"),
            start_bits: 1,
            stop_bits: 1,
            flow_control: FlowControl::None(),
        };

        let config3 = SerialStreamConfig {
            baud_rate: 115200,
            port_path: String::from("/dev/ttyUSB1"),
            start_bits: 1,
            stop_bits: 2,
            flow_control: FlowControl::XonXoff,
        };

        assert_eq!(config1, config2);
        assert_ne!(config1, config3);
    }
}
