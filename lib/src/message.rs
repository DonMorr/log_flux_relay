/// Represents a message with a timestamp, originator, and text content.
///
/// This struct encapsulates a message with a timestamp in milliseconds since the EPOC,
/// an originator string, and the text content of the message.
///
/// # Examples
///
/// 
/// use lib::src::message::Message;
///
/// let message = Message::new(1234567890, "Alice".to_string(), "Hello, world!".to_string());
/// 
///
/// # Fields
///
/// - `timestamp_ms`: The timestamp of the message in milliseconds since the EPOC.
/// - `originator`: The string representing the originator of the message.
/// - `text`: The text content of the message.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
/// A message with a timestamp, originator, and text content.
///
/// This struct represents a message with a timestamp in milliseconds since the EPOC,
/// an originator string, and the text content of the message.
pub struct Message {
    pub timestamp_ms: i64, // Number of milliseconds since EPOC.
    pub originator: String,
    pub text: String,
}

// Add new function called clear
impl Message {
    /// Creates a new `Message` with the given timestamp, originator, and text.
    ///
    /// This constructor function takes the timestamp in milliseconds since the EPOC,
    /// the originator string, and the text content of the message, and returns a
    /// new `Message` struct with those values.
    pub fn new(timestamp: i64, originator: String, text: String) -> Message {
        Message {
            timestamp_ms: timestamp,
            originator,
            text,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests the creation of a new `Message` instance.
    ///
    /// This test creates a new `Message` instance with a specific timestamp, originator, and text,
    /// and then verifies that the fields of the created `Message` match the expected values.
    fn test_message_creation() {
        let timestamp = 1234567890;
        let originator = "Alice".to_string();
        let text = "Hello, world!".to_string();

        let message = Message::new(timestamp, originator.clone(), text.clone());

        assert_eq!(message.timestamp_ms, timestamp);
        assert_eq!(message.originator, originator);
        assert_eq!(message.text, text);
    }

    #[test]
    /// Tests that two `Message` instances with the same timestamp, originator, and text
    /// are considered equal, while two `Message` instances with different timestamps
    /// are considered not equal.
    ///
    /// This test creates three `Message` instances, two with the same values and one
    /// with a different timestamp, and then verifies that the first two are equal and
    /// the third is not equal to the first.
    fn test_message_equality() {
        let message1 = Message::new(1234567890, "Bob".to_string(), "Test message".to_string());
        let message2 = Message::new(1234567890, "Bob".to_string(), "Test message".to_string());
        let message3 = Message::new(1234567891, "Bob".to_string(), "Test message".to_string());

        assert_eq!(message1, message2);
        assert_ne!(message1, message3);
    }

    #[test]
    /// Tests that cloning a `Message` instance produces a new instance with the same values.
    ///
    /// This test creates a `Message` instance, clones it, and then verifies that the original and
    /// cloned instances have the same values but different memory addresses.
    fn test_message_clone() {
        let original = Message::new(1234567890, "Charlie".to_string(), "Clone test".to_string());
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert!(!std::ptr::eq(&original, &cloned));
    }

    #[test]
    /// Tests that the `Debug` implementation for the `Message` struct produces the expected output.
    ///
    /// This test creates a `Message` instance with specific values for the timestamp, originator, and text
    /// fields, and then verifies that the debug output generated for that instance contains the expected
    /// values.
    fn test_message_debug_output() {
        let message = Message::new(1234567890, "Debug".to_string(), "Debug output test".to_string());
        let debug_output = format!("{:?}", message);

        assert!(debug_output.contains("1234567890"));
        assert!(debug_output.contains("Debug"));
        assert!(debug_output.contains("Debug output test"));
    }

    #[test]
    /// Tests that a `Message` can be created with empty fields.
    ///
    /// This test creates a `Message` instance with a timestamp of 0 and empty strings for the
    /// originator and text fields, and then verifies that the fields have the expected values.
    fn test_message_with_empty_fields() {
        let message = Message::new(0, String::new(), String::new());

        assert_eq!(message.timestamp_ms, 0);
        assert_eq!(message.originator, "");
        assert_eq!(message.text, "");
    }
}