use std::{sync::{atomic::{AtomicBool, Ordering}, mpsc::Sender, Arc}, thread::{self, JoinHandle}, time::Duration};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
extern crate mio;
extern crate mio_serial;
use super::{Stream, StreamConfig, StreamTypeConfig, Message, StreamCore};
use crate::{stream::INTERNAL_STREAM_TICK_MS, tools::waveforms_i2c::waveforms_i2c::WaveformsI2cControl};
use std::str;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WaveformsI2cStreamConfig {
    pub scl_pin: u8,
    pub sda_pin: u8,
    pub baud_rate: u32,
}

impl WaveformsI2cStreamConfig {
    pub fn new() -> Self {
        WaveformsI2cStreamConfig {
            scl_pin: 0,
            sda_pin: 1,
            baud_rate: 100000
        }
    }
}

#[derive(Debug)]
pub struct WaveformsI2cStream {
    core: StreamCore,
    config: StreamConfig,
    new_message_generated_sender: Sender<Message>,
    thread_handle: Option<JoinHandle<()>>,
    thread_stop_requsted: Arc<AtomicBool>
}

impl WaveformsI2cStream {
    pub fn new(config: StreamConfig) -> Result<Self, &'static str> {
        if let StreamTypeConfig::WaveformsI2c {..} = config.type_config {
            let core = StreamCore::new();
            Ok(Self{
                config:config,
                new_message_generated_sender: core.get_internal_input_sender_clone(),
                core: core,
                thread_handle: None,
                thread_stop_requsted: Arc::new(AtomicBool::new(false))
            })
        }
        else{
            Err("Invalid type_config for a WaveformsI2cStream")
        }
    }
}


impl Stream for WaveformsI2cStream { 
    fn start(&mut self) -> Result<(), String> {
        let stream_name = self.config.name.clone();
        let sender: Sender<Message> = self.new_message_generated_sender.clone();
        let scl_pin: u8;
        let sda_pin: u8;
        let baud_rate: u32;

        let stop_requested = Arc::clone(&self.thread_stop_requsted);
        
        println!("'{}' - WaveformsI2cStream starting thread", stream_name);

        if let StreamTypeConfig::WaveformsI2c {config} = &self.config.type_config {
            scl_pin = config.scl_pin;
            sda_pin = config.sda_pin;
            baud_rate = config.baud_rate;
        }
        else{
            return Err("Invalid type_config for a WaveformsI2cStream".to_string());
        }

        let control = WaveformsI2cControl::new(baud_rate, scl_pin, sda_pin).expect("Failed to create WaveformsI2cControl");

        self.thread_handle = Some(thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(INTERNAL_STREAM_TICK_MS));

            match control.read() {
                Ok(data) => {
                    let message = Message::new(Utc::now().timestamp_millis(),stream_name.clone(), data);
                    sender.send(message).expect("Failed to send message");
                },
                Err(_err) => {
                }
            }

            // Has stop been requested?
            if stop_requested.load(Ordering::Relaxed) {
                control.close().expect("Failed to close WaveformsI2cControl");
                break;
            }
        }));

        self.core.start()?;

        Ok(())
    }

    fn stop(&mut self) -> Result<(), String> {
        println!("'{}' - WaveformsI2cStream stopping", self.config.name);
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
