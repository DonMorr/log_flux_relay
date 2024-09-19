use std::{io::{self, Read}, sync::{atomic::{AtomicBool, Ordering}, mpsc::{Receiver, Sender}, Arc}, thread::{self, JoinHandle}, time::Duration};
use chrono::Utc;
use mio::{Events, Interest, Poll, Token};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
extern crate mio;
extern crate mio_serial;
use mio_serial::SerialPortBuilderExt;
use super::{Stream, StreamConfig, StreamTypeConfig, Message, StreamCore};
use crate::{stream::INTERNAL_STREAM_TICK_MS, tools::stream_tools::process_raw_log_entry};
use std::str;
const SERIAL_TOKEN: Token = Token(0);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FlowControl {
    None(),
    XonXoff,
    Etc,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SerialStreamConfig {
    // TODO - These may not be correct
    pub baud_rate: u32,
    pub port_path: String,
    pub start_bits: u8,
    pub stop_bits: u8,
    pub flow_control: FlowControl,
}

impl SerialStreamConfig {
    pub fn new() -> Self {
        SerialStreamConfig {
            baud_rate: 9600,
            port_path: String::from(""),
            start_bits: 1,
            stop_bits: 1,
            flow_control: FlowControl::None(),
        }
    }
}

#[derive(Debug)]
pub struct SerialStream {
    core: StreamCore,
    config: StreamConfig,
    new_message_generated_sender: Sender<Message>,
    new_message_received_receiver: Option<Receiver<Message>>,
    thread_handle: Option<JoinHandle<()>>,
    thread_stop_requsted: Arc<AtomicBool>
}

impl SerialStream {
    pub fn new(config: StreamConfig) -> Result<Self, &'static str> {
        if let StreamTypeConfig::Serial {..} = config.type_config {
            let mut core = StreamCore::new();
            Ok(Self{
                config:config,
                new_message_generated_sender: core.get_internal_input_sender_clone(),
                new_message_received_receiver: Some(core.get_internal_output_receiver()),
                core: core,
                thread_handle: None,
                thread_stop_requsted: Arc::new(AtomicBool::new(false))
            })
        }
        else{
            Err("Invalid type_config for a SerialStream")
        }
    }
}


impl Stream for SerialStream { 
    fn start(&mut self) -> Result<(), String> {
        let stream_name = self.config.name.clone();
        let receiver: Receiver<Message> = self.new_message_received_receiver.take().expect("Receiver unavailable");
        let sender: Sender<Message> = self.new_message_generated_sender.clone();
        let path:String;
        let baud_rate: u32;
        let mut buf = [0u8; 10240];
        let mut last_partial_line: String = String::new();
        let mut events = Events::with_capacity(1);
        let mut poll = match Poll::new() {
            Ok(poll) => poll,
            Err(e) => panic!("failed to create Poll instance; err={:?}", e),
        };

        let stop_requested = Arc::clone(&self.thread_stop_requsted);
        
        println!("'{}' - SerialStream starting thread", stream_name);

        if let StreamTypeConfig::Serial {config} = &self.config.type_config {
            path = config.port_path.clone();
            baud_rate = config.baud_rate;
        }
        else{
            todo!("Handle this error");
        }

        // Create the serial port
        println!("Opening {} at {},8N1", path, baud_rate);
        let mut rx = match mio_serial::new(path, baud_rate).open_native_async() {
            Ok(rx) => rx,
            Err(e) => panic!("failed to open serial port; err={:?}", e),
        };

        poll.registry()
            .register(&mut rx, SERIAL_TOKEN, Interest::READABLE)
            .unwrap();


        self.thread_handle = Some(thread::spawn(move || loop {
            
            // Has stop been requested?
            if stop_requested.load(Ordering::Relaxed) {
                break;
            }

            // New message received from core.
            while let Ok(msg) = receiver.try_recv() {
                todo!("write message to serial port");
            }
            
            match poll.poll(&mut events, Some(Duration::from_millis(INTERNAL_STREAM_TICK_MS))) {
                Ok(poll) => poll,
                Err(e) => panic!("failed to poll Poll instance; err={:?}", e),
            };

            // Process each event.
            for event in events.iter() {
                match event.token() {
                    SERIAL_TOKEN => loop {
                        match rx.read(&mut buf) {
                            Ok(count) => {
                                let raw_string = String::from_utf8_lossy(&buf[..count]);
                                
                                let (complete_lines,partial_line) = process_raw_log_entry(raw_string.to_string(), last_partial_line);
                                last_partial_line = partial_line;

                                for line in complete_lines {
                                    let new_msg: Message = Message::new(Utc::now().timestamp_millis(), stream_name.clone(), line);
                                    sender.send(new_msg);
                                }
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                break;
                            }
                            Err(e) => {
                                print!("Quitting due to read error: {}", e);
    
                            }
                        }
                    },
                    _ => {
                        // This should never happen as we only registered our
                        // `UdpSocket` using the `UDP_SOCKET` token, but if it ever
                        // does we'll log it.
                        // warn!("Got event for unexpected token: {:?}", event);
                    }
                }
            }
        }));

        self.core.start()?;

        Ok(())
    }

    fn stop(&mut self) -> Result<(), String> {
        println!("'{}' - SerialStream stopping", self.config.name);
        self.core.stop()?;
        self.await_thread_stop()
    }

    fn await_thread_stop(&mut self) -> Result<(), String> {
        self.thread_stop_requsted.store(true, Ordering::Relaxed);

        if let Some(thread_handle) = self.thread_handle.take() {
            thread_handle.join().expect("Failed to join thread");
            Ok(())
        } else {
            Err("Thread handle not available".to_string())
        }
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

    fn add_output(&mut self, receiver: Sender<Message>) -> Result<(), String>{
        self.core.add_external_output(receiver)
    }

    fn add_outputs(&mut self, senders: Vec<Sender<Message>>) -> Result<(), String>{
        self.core.add_external_outputs(senders)
    }
}
