use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Message {
    pub timestamp: u32,
    pub text: String,
}

impl Message {
    pub fn new(timestamp: u32, text: String) -> Message {
        Message {
            timestamp,
            text,
        }
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