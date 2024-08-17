
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

    pub fn process_raw_log_entry(raw_entry: String, mut last_partial_line: String) -> (Vec<String>, String) {
        let mut found_lines: Vec<String> = vec![];
        let line_incomplete: bool = !raw_entry.ends_with('\n');
        let mut lines = raw_entry.lines().peekable();
    
        // Handle the first line by combining it with last_partial_line
        if let Some(first_line) = lines.next() {
            let complete_first_line = last_partial_line + first_line;
            if lines.peek().is_some() || !line_incomplete {
                found_lines.push(complete_first_line);
                last_partial_line = String::new();
            } else {
                last_partial_line = complete_first_line;
            }
        }
    
        // Process the remaining lines
        while let Some(line) = lines.next() {
            if lines.peek().is_none() && line_incomplete {
                last_partial_line = String::from(line);
            } else {
                found_lines.push(String::from(line));
            }
        }
    
        (found_lines, last_partial_line)
    }

}


#[cfg(test)]
mod tests {

    use crate::log_flux_relay::{SerialConfiguration, FileConfig, FlowControl, SocketConfiguration, RelayConfig, Stream, process_raw_log_entry};

    #[test]
    fn process_raw_log_entry_test_simple_log(){
        let expected_lines: Vec<String> = vec![String::from("first"), 
                                                String::from("second"),
                                                String::from("third")];
        let expected_partial_line: String = String::new();

        let entry:String = String::from("first\r\nsecond\r\nthird\r\n");
        let last_partial_line:String = String::new();

        let (lines,partial_line) = process_raw_log_entry(entry, last_partial_line);

        assert!(expected_lines == lines);
        assert!(expected_partial_line == partial_line);
    }

    #[test]
    fn process_raw_log_entry_test_last_with_no_newling(){
        let expected_lines: Vec<String> = vec![String::from("first"), 
                                                String::from("second")];
        let expected_partial_line: String = String::from("third");

        let entry:String = String::from("first\r\nsecond\r\nthird");
        let last_partial_line:String = String::new();

        let (lines,partial_line) = process_raw_log_entry(entry, last_partial_line);

        assert!(expected_lines == lines);
        assert!(expected_partial_line == partial_line);
    }

    #[test]
    fn process_raw_log_entry_test_last_partial_line_not_empty(){
        let expected_partial_line: String = String::from("fourth");
        let expected_lines: Vec<String> = vec![String::from("start_first"),
                                                String::from("second"), 
                                                String::from("third")];

        let last_partial_line:String = String::from("start_");
        let entry:String = String::from("first\r\nsecond\r\nthird\r\nfourth");

        println!(">>> Entry: {}", entry);
        println!(">>> Start partial line: {}", last_partial_line);
        let (lines,partial_line) = process_raw_log_entry(entry, last_partial_line);
        println!("Expected Lines: {:?}", expected_lines);
        println!("Actual lines: {:?}", lines);
        assert!(expected_lines == lines);
        assert!(expected_partial_line == partial_line);
        assert!(expected_lines == lines);
    }

    #[test]
    fn process_raw_log_entry_test_last_partial_line_no_lines(){
        let expected_partial_line: String = String::from("start_first");
        let expected_lines: Vec<String> = vec![];

        let last_partial_line:String = String::from("start_");
        let entry:String = String::from("first");

        println!(">>> Entry: {}", entry);
        println!(">>> Start partial line: {}", last_partial_line);
        let (lines,partial_line) = process_raw_log_entry(entry, last_partial_line);

        println!(">>> Expected partial line: {}", expected_partial_line);
        println!(">>> Actual partial line: {}", partial_line);
        println!(">>> Expected lines: {:?}", expected_lines);
        println!(">>> Actual lines: {:?}", lines);

        assert!(expected_lines == lines);
        assert!(expected_partial_line == partial_line);
    }

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
