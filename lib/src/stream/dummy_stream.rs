use std::{sync::mpsc::{Receiver, Sender}, thread::{self, JoinHandle}, time::Duration};
use chrono::{Utc, Local, TimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::stream::INTERNAL_STREAM_TICK_MS;
use super::{Stream, StreamConfig, StreamTypeConfig, Message, StreamCore};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DummyStreamConfig{
    pub inter_message_generation_period_ms: u64,
    pub generates_messages: bool,
    pub print_to_standard_out: bool
}

impl DummyStreamConfig {
    pub fn new() -> Self {
        DummyStreamConfig {inter_message_generation_period_ms: 1000, generates_messages: false, print_to_standard_out: true}
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
        let generates_messages: bool;
        let message_generation_period_ms: u64;
        let prints_to_standard_out: bool;
        let mut counter: u64 = 0;
        let mut msg_counter: i32 = 0;

        if let StreamTypeConfig::Dummy {config} = &self.config.type_config {
            generates_messages = config.generates_messages;
            message_generation_period_ms = config.inter_message_generation_period_ms;
            prints_to_standard_out = config.print_to_standard_out;
        }
        else{
            todo!("Handle this error");
        }
        
        println!("'{}' - DummyStream starting thread", stream_name);

        self.thread_handle = Some(thread::spawn(move || loop {

            // Handle Message received from core
            while let Ok(msg) = receiver.try_recv() {
                if prints_to_standard_out {
                    let datetime = Local.timestamp_millis_opt(msg.timestamp_ms);
                    let formatted_datetime = datetime.single().map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()).unwrap_or_else(|| "Invalid timestamp".to_string());
                    let ms = msg.timestamp_ms%1000;
                    let originator = msg.originator;
                    let text = msg.text;
                    println!("'{stream_name}' - {formatted_datetime}:{ms:0>3} - '{originator}' - '{text}'");
                }
            }
            
            if generates_messages{
                counter = counter + INTERNAL_STREAM_TICK_MS;

                if counter % message_generation_period_ms == 0 {
                    msg_counter += 1;
                    let new_msg: Message = Message::new(Utc::now().timestamp_millis(), stream_name.clone(), format!("New message {}", msg_counter));
                    
                    if prints_to_standard_out {
                        println!("'{}' - DummyStream generated new message: {} at time {}", stream_name, new_msg.text, new_msg.timestamp_ms);
                    }

                    sender.send(new_msg);
                }
            }

            thread::sleep(Duration::from_millis(INTERNAL_STREAM_TICK_MS));
        }));

        self.core.start();

        true
    }
    fn stop(&mut self) -> bool {
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
