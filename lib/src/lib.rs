
mod log_flux_relay{

use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub enum FlowControl{
        None(),
        XonXoff,
        Etc
    }

    #[derive(Serialize, Deserialize)]
    pub struct SerialConfiguration{
        pub baud_rate: u32,
        pub port_path: String,
        pub start_bits: u8,
        pub stop_bits:u8,
        pub flow_control:FlowControl
    }

    #[derive(Serialize, Deserialize)]
    pub struct SocketConfiguration{
        pub ip_address: String,
        pub port: u16
    }
    

    #[derive(Serialize, Deserialize)]
    pub struct SystemConfig{
        // todo
    }
    
    #[derive(Serialize, Deserialize)]
    pub struct FileConfig{
        pub file_name: String
    }
    
    #[derive(Serialize, Deserialize)]
    pub struct TerminalConfig{
    
    }
    
    #[derive(Serialize, Deserialize)]
    pub struct MqttConfig{
        // todo
    }


    #[derive(Serialize, Deserialize)]
    pub struct BufferConfig{
        // todo
    }
    
    #[derive(Serialize, Deserialize)]
    pub enum Stream{
        Serial{config: SerialConfiguration},
        Socket{config: SocketConfiguration},
        System{config: SystemConfig},
        File{config: FileConfig},
        Terminal{config: TerminalConfig},
        Mqtt{config: MqttConfig},
        Buffer{config: BufferConfig}
    }

    #[derive(Serialize, Deserialize)]
    pub struct RelayConfig{
        pub input_streams: Vec<Stream>,
        pub output_streams: Vec<Stream>
    }

    // pub fn load_config(file_path: String) -> Config {
    //     let config:Config = Config.new
    // }

    // pub fn save_config(filepath: String, config:Config){

    // }

}





pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {

    use crate::log_flux_relay::FileConfig;

    use super::log_flux_relay::{SerialConfiguration, FlowControl, SocketConfiguration, RelayConfig, Stream};

    #[test]
    fn it_works() {
        let serial_config:SerialConfiguration = SerialConfiguration {
            baud_rate: 9600,
            port_path: String::from("/dev/ttyUSB0"),
            start_bits: 1,
            stop_bits: 1,
            flow_control: FlowControl::XonXoff,
        };

        let sock_config: SocketConfiguration = SocketConfiguration{
            ip_address: String::from("192.168.9.1"),
            port: 5693
        };

        let file_config_1: FileConfig = FileConfig {
            file_name: String::from("test_file_1.txt")
        };

        let file_config_2: FileConfig = FileConfig {
            file_name: String::from("test_file_2.txt")
        };

        let mut config: RelayConfig = RelayConfig{
            input_streams: vec![],
            output_streams: vec![]
        };

        config.input_streams.push(Stream::Serial { config: serial_config });
        config.input_streams.push(Stream::Socket { config: sock_config });

        config.output_streams.push(Stream::File { config: file_config_1 });
        config.output_streams.push(Stream::File { config: file_config_2 });

        println!("Config: {}", serde_json::to_string(&config).unwrap());
        
    }
}
