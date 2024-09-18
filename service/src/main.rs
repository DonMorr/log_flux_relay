use std::{net::UdpSocket, thread, time::Duration};

use lib::{
    stream::{
        dummy_stream::DummyStreamConfig, 
        file_stream::{self, FileStreamConfig}, 
        serial_stream::{FlowControl, SerialStreamConfig},
        udp_stream::UdpStreamConfig,
        Direction, StreamConfig, StreamTypeConfig
    }, 
    yalm_engine::YalmEngine};



fn main() {
    // let mut dummy_stream_config_gen: DummyStreamConfig = DummyStreamConfig::new();
    // dummy_stream_config_gen.generates_messages = true;
    // dummy_stream_config_gen.inter_message_generation_period_ms = 10;
    // dummy_stream_config_gen.print_to_standard_out = false;

    // let mut generator_stream_config: StreamConfig = StreamConfig::default();
    // generator_stream_config.name = String::from("Generator Stream");
    // generator_stream_config.direction = Direction::Output;
    // generator_stream_config.type_config = StreamTypeConfig::Dummy { config:dummy_stream_config_gen };

    // let dummy_stream_config_con: DummyStreamConfig = DummyStreamConfig::new();
    // let mut consumer_stream_config_a: StreamConfig = StreamConfig::default();
    // consumer_stream_config_a.name = String::from("Consumer Stream A");
    // consumer_stream_config_a.direction = Direction::Input;
    // consumer_stream_config_a.type_config = StreamTypeConfig::Dummy { config:dummy_stream_config_con };
    // consumer_stream_config_a.input_filter = String::from("tick");

    // let mut dummy_stream_config_con: DummyStreamConfig = DummyStreamConfig::new();
    // dummy_stream_config_con.print_to_standard_out = false;
    // let mut consumer_stream_config_b: StreamConfig = StreamConfig::default();
    // consumer_stream_config_b.name = String::from("Consumer Stream B");
    // consumer_stream_config_b.direction = Direction::Input;
    // consumer_stream_config_b.type_config = StreamTypeConfig::Dummy { config:dummy_stream_config_con };
    // consumer_stream_config_b.input_filter = String::from("tock");

    // generator_stream_config.output_streams.push(consumer_stream_config_a.uuid.clone());
    // generator_stream_config.output_streams.push(consumer_stream_config_b.uuid.clone());

    let mut serial_stream_1_config: SerialStreamConfig = SerialStreamConfig::new();
    serial_stream_1_config.baud_rate = 115200;
    serial_stream_1_config.port_path = String::from("COM6");
    serial_stream_1_config.start_bits = 1;
    serial_stream_1_config.stop_bits = 1;
    serial_stream_1_config.flow_control = FlowControl::None();

    let mut source_stream_1_config: StreamConfig = StreamConfig::default();
    source_stream_1_config.name = String::from("Serial Stream COM6");
    source_stream_1_config.direction = Direction::BiDirectional;
    source_stream_1_config.type_config = StreamTypeConfig::Serial { config:serial_stream_1_config };

    // let mut serial_stream_2_config: SerialStreamConfig = SerialStreamConfig::new();
    // serial_stream_2_config.baud_rate = 115200;
    // serial_stream_2_config.port_path = String::from("COM6");
    // serial_stream_2_config.start_bits = 1;
    // serial_stream_2_config.stop_bits = 1;
    // serial_stream_2_config.flow_control = FlowControl::None();

    // let mut source_stream_2_config: StreamConfig = StreamConfig::default();
    // source_stream_2_config.name = String::from("Serial Stream COM6");
    // source_stream_2_config.direction = Direction::BiDirectional;
    // source_stream_2_config.type_config = StreamTypeConfig::Serial { config:serial_stream_2_config };

    let dummy_stream_config_con: DummyStreamConfig = DummyStreamConfig::new();
    let mut output_dummy_stream_config: StreamConfig = StreamConfig::default();
    output_dummy_stream_config.name = String::from("Dummy Stream A");
    output_dummy_stream_config.direction = Direction::Input;
    output_dummy_stream_config.type_config = StreamTypeConfig::Dummy { config:dummy_stream_config_con };

    let file_stream_config: FileStreamConfig = FileStreamConfig::new(String::from("simple_text.txt"));
    let mut output_file_stream_config: StreamConfig = StreamConfig::default();
    output_file_stream_config.name = String::from("File Writer A");
    output_file_stream_config.direction = Direction::Output;
    output_file_stream_config.type_config = StreamTypeConfig::File { config:file_stream_config };

    // UDP stream for sending data to external ULogViewer
    let mut udp_stream_config: UdpStreamConfig = UdpStreamConfig::new();
    udp_stream_config.output_ip_address = String::from("localhost");
    udp_stream_config.output_port = 49152;
    udp_stream_config.output_enabled = true;
    let mut output_udp_stream_config: StreamConfig = StreamConfig::default();
    output_udp_stream_config.name = String::from("UDP Writer A");
    output_udp_stream_config.direction = Direction::Output;
    output_udp_stream_config.type_config = StreamTypeConfig::Udp { config: udp_stream_config };

    // Connect up the streams
    source_stream_1_config.output_streams.push(output_dummy_stream_config.uuid.clone());
    //source_stream_2_config.output_streams.push(output_dummy_stream_config.uuid.clone());
    output_dummy_stream_config.output_streams.push(output_file_stream_config.uuid.clone());
    output_dummy_stream_config.output_streams.push(output_udp_stream_config.uuid.clone());

    let mut engine: YalmEngine = YalmEngine::new();
    engine.add_stream(source_stream_1_config);
    //engine.add_stream(source_stream_2_config);
    engine.add_stream(output_file_stream_config);
    engine.add_stream(output_dummy_stream_config);
    engine.add_stream(output_udp_stream_config);

    if engine.initialise(){
        println!("Engine successfully initialised");
    }

    if engine.start() {
        println!("Engine successfully started");

        thread::sleep(Duration::from_secs(60*60));

        engine.stop();
    }
    else {
        println!("Failed to start, validation failed");
    }

}


#[cfg(test)]
mod tests {
}