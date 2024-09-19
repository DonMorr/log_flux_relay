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
