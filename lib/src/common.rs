

use core::time;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FlowControl{
    None(),
    XonXoff,
    Etc
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SerialConfiguration{
    pub baud_rate: u32,
    pub port_path: String,
    pub start_bits: u8,
    pub stop_bits:u8,
    pub flow_control:FlowControl
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SocketConfiguration{
    pub ip_address: String,
    pub port: u16
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SystemConfig{
    // todo
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FileConfig{
    pub file_name: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TerminalConfig{

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MqttConfig{
    // todo
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BufferConfig{
    // todo
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Stream{
    Serial{config: SerialConfiguration},
    Socket{config: SocketConfiguration},
    System{config: SystemConfig},
    File{config: FileConfig},
    Terminal{config: TerminalConfig},
    Mqtt{config: MqttConfig},
    Buffer{config: BufferConfig}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RelayConfig{
    pub input_streams: Vec<Stream>,
    pub output_streams: Vec<Stream>
}

pub struct Message{
    pub timestamp: u32,
    pub text: String
}

impl Message{
    pub fn new(timestamp:u32, text: String)-> Message{
     Message { timestamp: timestamp, text: text}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_message_creation() {
        let timestamp = 1234567890;
        let text = String::from("Hello, world!");
        let message = Message::new(timestamp, text.clone());

        assert_eq!(message.timestamp, timestamp);
        assert_eq!(message.text, text);
    }

    #[test]
    fn test_new_message_with_empty_text() {
        let timestamp = 96543210;
        let text = String::new();
        let message = Message::new(timestamp, text);

        assert_eq!(message.timestamp, timestamp);
        assert_eq!(message.text, "");
    }

    #[test]
    fn test_new_message_with_zero_timestamp() {
        let timestamp = 0;
        let text = String::from("Zero timestamp");
        let message = Message::new(timestamp, text.clone());

        assert_eq!(message.timestamp, 0);
        assert_eq!(message.text, text);
    }

    #[test]
    fn test_new_message_with_max_timestamp() {
        let timestamp = u32::MAX;
        let text = String::from("Max timestamp");
        let message = Message::new(timestamp, text.clone());

        assert_eq!(message.timestamp, u32::MAX);
        assert_eq!(message.text, text);
    }
}