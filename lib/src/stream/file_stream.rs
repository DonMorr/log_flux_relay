use std::{sync::mpsc::{Receiver, Sender}, thread::{self, JoinHandle}, time::Duration};
use chrono::{Local, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::stream::INTERNAL_STREAM_TICK_MS;
use super::{Stream, StreamConfig, StreamTypeConfig, Message, StreamCore};
use std::fs::File;
use std::io::Write;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FileStreamConfig {
    pub file_path: String,
}

impl FileStreamConfig {
    pub fn new(file_name: String) -> Self {
        FileStreamConfig {file_path: file_name}
    }
}

#[derive(Debug)]
pub struct FileStream {
    config: StreamConfig,
    core: StreamCore,
    new_message_generated_sender: Sender<Message>,
    new_message_received_receiver: Option<Receiver<Message>>,
    thread_handle: Option<JoinHandle<()>>
}

impl Stream for FileStream {

    fn start(&mut self) -> Result<(), String> {
        let stream_name = self.config.name.clone();
        let receiver: Receiver<Message> = self.new_message_received_receiver.take().expect("Receiver unavailable");
        let file_path: String;
        let full_file_path: String;

        if let StreamTypeConfig::File {config} = &self.config.type_config {
            file_path = config.file_path.clone();
        }
        else{
            todo!("Handle this error");
        }

        let datetime = Local.timestamp_millis_opt(Utc::now().timestamp_millis());
        let formatted_datetime = datetime.single().map(|dt| dt.format("%Y-%m-%d_%H%M%S").to_string()).unwrap_or_else(|| "Invalid timestamp".to_string());
        full_file_path = format!("{formatted_datetime}_{file_path}");
        
        println!("'{}' - FileStream starting thread", stream_name);
        let mut file = match File::create(full_file_path.clone()) {
            Ok(file) => file,
            Err(e) => panic!("Error opening file: {}", e),
        };
        println!("File opened: '{full_file_path}'");

        let log_message = format!("'{stream_name}'");
        writeln!(file, "{}", log_message).expect("Failed to write to file");

        self.thread_handle = Some(thread::spawn(move || loop {

            // Handle Message received from core
            while let Ok(msg) = receiver.try_recv() {
                let datetime = Local.timestamp_millis_opt(msg.timestamp_ms);
                let formatted_datetime = datetime.single().map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()).unwrap_or_else(|| "Invalid timestamp".to_string());
                let ms = msg.timestamp_ms%1000;
                let originator = msg.originator;
                let text = msg.text;
                let log_message = format!("'{originator}' - {formatted_datetime}:{ms:0>3} - '{text}'");
                writeln!(file, "{}", log_message).expect("Failed to write to file");
            }
            
            thread::sleep(Duration::from_millis(INTERNAL_STREAM_TICK_MS));
        }));

        self.core.start();
        todo!("Implement start");
    }
    fn stop(&mut self) -> Result<(), String> {
        todo!("Implement stop");
    }

    fn get_config(&self) -> &StreamConfig {
        &self.config
    }

    fn get_status(&self) -> &StreamCore {
        &self.core
    }

    fn get_uuid(&self) -> &Uuid{
        &self.config.uuid
    }

    fn add_output(&mut self, receiver: Sender<Message>){
        self.core.add_external_output(receiver);
    }

    fn add_outputs(&mut self, senders: Vec<Sender<Message>>){
        self.core.add_external_outputs(senders);
    }

}

impl FileStream {
    pub fn new(config: StreamConfig) -> Result<Self, &'static str> {
        if let StreamTypeConfig::File {..} = config.type_config {
            let mut core = StreamCore::new();

            Ok(Self{
                config: config,
                new_message_generated_sender: core.get_internal_input_sender_clone(),
                new_message_received_receiver: Some(core.get_internal_output_receiver()),
                core: core,
                thread_handle: None
            })
        }
        else{
            Err("Invalid type_config for a FileStream")
        }
    }
}