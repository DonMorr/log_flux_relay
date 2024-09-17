
pub mod yalm_engine;
pub mod yalm_config;
pub mod filter;
pub mod message;
pub mod stream;

pub mod log_flux_relay{
    
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

    use crate::{log_flux_relay::process_raw_log_entry};

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
}
