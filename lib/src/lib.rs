
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

    pub fn process_raw_log_entry(raw_entry: String, mut last_partial_line: String)-> (Vec<String>, String){
        let mut found_lines:Vec<String> = vec![];
        let line_incomplete: bool = !raw_entry.ends_with('\n');
        let mut it: std::iter::Peekable<std::str::Lines> = raw_entry.lines().peekable();

        while let Some(line) = it.next()  {
            // Is it the last line?
            if it.peek().is_none() {
                if line_incomplete {
                    last_partial_line = last_partial_line + line;
                }
                else {
                    found_lines.push(String::from(line));
                }
            }
            else{
                found_lines.push(String::from(line));
            }
        }

        (found_lines, last_partial_line)
    }

}





pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {

    use crate::log_flux_relay::FileConfig;

    use super::log_flux_relay::{SerialConfiguration, FlowControl, SocketConfiguration, RelayConfig, Stream, process_raw_log_entry};

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
        let expected_partial_line: String = String::new();
        let expected_lines: Vec<String> = vec![String::from("start_first"),
                                                String::from("second"), 
                                                String::from("third")];

        let last_partial_line:String = String::from("start_");
        let entry:String = String::from("first\r\nsecond\r\nthird\r\n");

        let (lines,partial_line) = process_raw_log_entry(entry, last_partial_line);

        assert!(expected_lines == lines);
        assert!(expected_partial_line == partial_line);
    }

    #[test]
    fn process_raw_log_entry_test_last_partial_line_no_lines(){
        let expected_partial_line: String = String::from("start_first");
        let expected_lines: Vec<String> = vec![];

        let last_partial_line:String = String::from("start_");
        let entry:String = String::from("first");

        let (lines,partial_line) = process_raw_log_entry(entry, last_partial_line);

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
