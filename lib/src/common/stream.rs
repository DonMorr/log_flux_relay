use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FlowControl {
    None(),
    XonXoff,
    Etc,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SerialConfiguration {
    pub baud_rate: u32,
    pub port_path: String,
    pub start_bits: u8,
    pub stop_bits: u8,
    pub flow_control: FlowControl,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SocketConfiguration {
    pub ip_address: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SystemConfig {
    // todo
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FileConfig {
    pub file_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TerminalConfig {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MqttConfig {
    // todo
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BufferConfig {
    // todo
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Direction {
    Input,
    Output,
    BiDirectional,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DataType {
    Ascii,
    Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StreamInfo {
    pub uuid: Uuid,
    pub name: String,
    pub direction: Direction,
    pub data_type: DataType,
    pub output_streams: Vec<Uuid>,
    pub input_filter: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Stream {
    Serial {
        info: StreamInfo,
        config: SerialConfiguration,
    },
    Socket {
        info: StreamInfo,
        config: SocketConfiguration,
    },
    System {
        info: StreamInfo,
        config: SystemConfig,
    },
    File {
        info: StreamInfo,
        config: FileConfig,
    },
    Terminal {
        info: StreamInfo,
        config: TerminalConfig,
    },
    Mqtt {
        info: StreamInfo,
        config: MqttConfig,
    },
    Buffer {
        info: StreamInfo,
        config: BufferConfig,
    },
}