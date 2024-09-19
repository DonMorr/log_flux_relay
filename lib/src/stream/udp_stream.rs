use std::{borrow::Borrow, sync::{atomic::{AtomicBool, Ordering}, mpsc::{Receiver, Sender}, Arc}, thread::{self, JoinHandle}, time::Duration};
use chrono::{Utc, Local, TimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::stream::INTERNAL_STREAM_TICK_MS;
use super::{Stream, StreamConfig, StreamTypeConfig, Message, StreamCore};
use std::net::UdpSocket;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UdpStreamConfig{
    pub output_ip_address: String,
    pub output_port: u16,
    pub output_enabled: bool,
    
    pub input_port: u16,
    pub input_enabled: bool
}

impl UdpStreamConfig {
    pub fn new() -> Self {
        UdpStreamConfig {output_port: 0, output_enabled: false, output_ip_address: String::new(), input_port: 0, input_enabled: false}
    }
}

#[derive(Debug)]
pub struct UdpStream {
    config: StreamConfig,
    core: StreamCore,
    new_message_generated_sender: Sender<Message>,
    new_message_received_receiver: Option<Receiver<Message>>,
    thread_handle: Option<JoinHandle<()>>,
    thread_stop_requsted: Arc<AtomicBool>
}

impl Stream for UdpStream {

    fn start(&mut self) -> Result<(), String> {
        let stream_name = self.config.name.clone();
        let receiver: Receiver<Message> = self.new_message_received_receiver.take().expect("Receiver unavailable");
        let sender: Sender<Message> = self.new_message_generated_sender.clone();

        let out_ip_address: String;
        let out_port: u16;
        let out_enabled: bool;
        let mut out_socket: Option<UdpSocket> = None;
        let mut out_address: String = String::new();
        let stop_requested = Arc::clone(&self.thread_stop_requsted);

        let in_port: u16;
        let in_enabled: bool;

        if let StreamTypeConfig::Udp {config} = &self.config.type_config {
            out_ip_address = config.output_ip_address.clone();
            out_port = config.output_port;
            out_enabled = config.output_enabled;
            in_port = config.input_port;
            in_enabled = config.input_enabled;
        }
        else{
            todo!("Handle this error");
        }
        
        println!("'{}' - UdpStream starting thread", stream_name);

        if out_enabled{
            out_address = format!("{out_ip_address}:{out_port}");

            out_socket = match UdpSocket::bind("0.0.0.0:0") {
                Ok(out_socket) => Some(out_socket),
                Err(e) => panic!("failed to open port; err={:?}", e),
            };
        }

        if in_enabled {
            todo!("Support for input UDP server currently not suppored.");
        }

        self.thread_handle = Some(thread::spawn(move || loop {

            if out_enabled {
                match out_socket {
                    Some(ref socket) => {
                        while let Ok(msg) = receiver.try_recv() {
                            let originator = msg.originator;
                            let text = msg.text;
                            let timestamp = msg.timestamp_ms;
                            let log_message = format!("'{originator}' - {timestamp} - '{text}'\n");
                            match socket.send_to(log_message.as_bytes(), out_address.clone()) {
                                Err(e) => {
                                    eprintln!("Failed to send message: {}", e);
                                },
                                _ =>{}
                            }
                        }
                    },
                    _ => {
                    },
                }
            }
            
            
            thread::sleep(Duration::from_millis(INTERNAL_STREAM_TICK_MS));
                        
            // Has stop been requested?
            if stop_requested.load(Ordering::Relaxed) {
                break;
            }
        }));

        self.core.start()?;

        Ok(())
    }

    fn stop(&mut self) -> Result<(), String> {
        println!("'{}' - UdpStream stopping", self.config.name);
        self.core.stop()?;
        self.await_thread_stop()?;
        Ok(())
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

impl UdpStream {
    pub fn new(config: StreamConfig) -> Result<Self, &'static str> {
        if let StreamTypeConfig::Udp {..} = config.type_config {
            let mut core = StreamCore::new();

            Ok(Self{
                config: config,
                new_message_generated_sender: core.get_internal_input_sender_clone(),
                new_message_received_receiver: Some(core.get_internal_output_receiver()),
                core: core,
                thread_handle: None,
                thread_stop_requsted: Arc::new(AtomicBool::new(false))
            })
        }
        else{
            Err("Invalid type_config for a UdpStream")
        }
    }
}
