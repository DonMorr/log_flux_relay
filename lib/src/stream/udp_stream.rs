use std::{net::Shutdown, sync::{atomic::{AtomicBool, Ordering}, mpsc::{Receiver, Sender}, Arc}, thread::{self, JoinHandle}, time::Duration};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::stream::INTERNAL_STREAM_TICK_MS;
use super::{Stream, StreamConfig, StreamTypeConfig, Message, StreamCore};
use std::net::UdpSocket;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UdpDirection{
    UdpOutput,
    UdpInput
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UdpStreamConfig{
    pub direction: UdpDirection,
    pub output_ip_address: String,
    pub output_port: u16,
    pub input_port: u16,
}

impl UdpStreamConfig {
    pub fn new() -> Self {
        UdpStreamConfig {output_port: 0, direction: UdpDirection::UdpOutput, output_ip_address: String::new(), input_port: 0}
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

        let direction: UdpDirection;
        let out_ip_address: String;
        let out_port: u16;
        let mut out_socket: Option<UdpSocket> = None;
        let mut out_address: String = String::new();
        let stop_requested = Arc::clone(&self.thread_stop_requsted);
        let mut in_socket: Option<UdpSocket> = None;

        let in_port: u16;
        let mut critical_error_occurred: bool = false;

        if let StreamTypeConfig::Udp {config} = &self.config.type_config {
            out_ip_address = config.output_ip_address.clone();
            out_port = config.output_port;
            in_port = config.input_port;
            direction = config.direction.clone();
        }
        else{
            todo!("Handle this error");
        }
        
        println!("'{}' - UdpStream starting thread", stream_name);

        if direction == UdpDirection::UdpOutput {
            out_address = format!("{out_ip_address}:{out_port}");
            println!("UdpStream - output enabled, sending to {out_address}");

            out_socket = match UdpSocket::bind("0.0.0.0:0") {
                Ok(out_socket) => Some(out_socket),
                Err(e) => panic!("failed to open port; err={:?}", e)
            };
        }
        else {
            println!("UdpStream - input enabled, listening on port {in_port}");
            in_socket = match UdpSocket::bind(format!("0.0.0.0:{in_port}")) {
                Ok(in_socket) => Some(in_socket),
                Err(e) => panic!("failed to open port; err={:?}", e)
            };
        }

        self.thread_handle = Some(thread::Builder::new().name(stream_name.clone()).spawn(move || loop {

            if direction == UdpDirection::UdpInput {
                let mut buf = [0; 1024];
                match in_socket.as_ref().unwrap().recv_from(&mut buf) {
                    Ok((size, _)) => {
                        let received_message = String::from_utf8_lossy(&buf[..size]);
                        let timestamp = Utc::now().timestamp_millis();
                        let message = Message {
                            originator: stream_name.clone(),
                            text: received_message.to_string(),
                            timestamp_ms: timestamp
                        };
                        sender.send(message).expect(&format!("{stream_name} - Failed to send message"));
                    },
                    Err(e) => {
                        eprintln!("Failed to receive message: {}", e);
                        break;
                    }
                }
            }
            else {
                match out_socket {
                    Some(ref socket) => {
                        while let Ok(msg) = receiver.try_recv() {
                            let originator = msg.originator;
                            let text = msg.text;
                            let timestamp = msg.timestamp_ms;
                            let log_message = format!("'{originator}' - {timestamp} - '{text}'\n");
                            match socket.send_to(log_message.as_bytes(), out_address.clone()) {
                                Err(e) => {
                                    eprintln!("{}", format!("{stream_name} - Failed to send message - {}", e.to_string()));
                                    critical_error_occurred = true;
                                    break;
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
            if critical_error_occurred || stop_requested.load(Ordering::Relaxed) {
                break;
            }
        }).map_err(|e| e.to_string())?);

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
