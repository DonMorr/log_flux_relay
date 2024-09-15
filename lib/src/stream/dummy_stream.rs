use std::{str::FromStr, sync::mpsc::{Receiver, Sender}, thread::{self, JoinHandle}, time::Duration};
use chrono::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::from_str;
use uuid::Uuid;
use super::{Stream, StreamConfig, StreamTypeConfig, Message, StreamCore};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DummyStreamConfig{
    pub message_generation_rate_hz: u16,
    pub generates_messages: bool
}

impl DummyStreamConfig {
    pub fn new() -> Self {
        DummyStreamConfig {message_generation_rate_hz: 1, generates_messages: true}
    }
}

#[derive(Debug)]
pub struct DummyStream {
    config: StreamConfig,
    core: StreamCore,
    new_message_generated_sender: Sender<Message>,
    new_message_received_receiver: Option<Receiver<Message>>,
    thread_handle: Option<JoinHandle<()>>
}

impl Stream for DummyStream {

    fn start(&mut self) -> bool {
        let stream_name = self.config.name.clone();
        let receiver: Receiver<Message> = self.new_message_received_receiver.take().expect("Receiver unavailable");
        let sender: Sender<Message> = self.new_message_generated_sender.clone();
        let mut counter: u16 = 0;
        let mut msg_counter: i32 = 0;
        let mut generates_messages: bool;

        if let StreamTypeConfig::Dummy {config} = &self.config.type_config {
            generates_messages = config.generates_messages;
        }
        else{
            todo!("Handle this error");
        }
        
        if generates_messages {
            println!("{} - DummyStream generates messages", stream_name);
        }
        
        println!("{} - DummyStream starting thread", stream_name);

        self.thread_handle = Some(thread::spawn(move || loop {

            // Handle Message received from core
            while let Ok(msg) = receiver.try_recv() {
                println!("{} - DummyStream received Message: {}", stream_name, msg.text)
            }
            
            if generates_messages{
                counter += 1;
                if counter >= 100 {
                    counter = 0;
                    msg_counter += 1;
                    let new_msg: Message = Message::new(Utc::now().timestamp(), format!("New message {} from '{}'", msg_counter, stream_name));
                    println!("{} - DummyStream generated new message: {}", stream_name, new_msg.text);
                    sender.send(new_msg);
                }
            }

            thread::sleep(Duration::from_millis(10));
        }));

        self.core.start();

        true
    }
    fn stop(&mut self) -> bool {
        todo!("Implement stop");
        false
    }

    fn pause(&mut self) -> bool {
        todo!("Implement pause");
        false
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

impl DummyStream {
    pub fn new(config: StreamConfig) -> Result<Self, &'static str> {
        if let StreamTypeConfig::Dummy {..} = config.type_config {
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
            Err("Invalid type_config for a DummyStream")
        }
    }
}
