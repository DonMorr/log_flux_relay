use std::{thread, time::Duration};

use lib::{stream::{dummy_stream::DummyStreamConfig, Direction, StreamConfig, StreamTypeConfig}, yalm_engine::YalmEngine};



fn main() {
    let mut dummy_stream_config_gen: DummyStreamConfig = DummyStreamConfig::new();
    dummy_stream_config_gen.generates_messages = true;
    dummy_stream_config_gen.inter_message_generation_period_ms = 10;
    dummy_stream_config_gen.print_to_standard_out = false;

    let mut generator_stream_config: StreamConfig = StreamConfig::default();
    generator_stream_config.name = String::from("Generator Stream");
    generator_stream_config.direction = Direction::Output;
    generator_stream_config.type_config = StreamTypeConfig::Dummy { config:dummy_stream_config_gen };

    let dummy_stream_config_con: DummyStreamConfig = DummyStreamConfig::new();
    let mut consumer_stream_config_a: StreamConfig = StreamConfig::default();
    consumer_stream_config_a.name = String::from("Consumer Stream A");
    consumer_stream_config_a.direction = Direction::Input;
    consumer_stream_config_a.type_config = StreamTypeConfig::Dummy { config:dummy_stream_config_con };
    consumer_stream_config_a.input_filter = String::from("tick");

    let mut dummy_stream_config_con: DummyStreamConfig = DummyStreamConfig::new();
    dummy_stream_config_con.print_to_standard_out = false;
    let mut consumer_stream_config_b: StreamConfig = StreamConfig::default();
    consumer_stream_config_b.name = String::from("Consumer Stream B");
    consumer_stream_config_b.direction = Direction::Input;
    consumer_stream_config_b.type_config = StreamTypeConfig::Dummy { config:dummy_stream_config_con };
    consumer_stream_config_b.input_filter = String::from("tock");

    generator_stream_config.output_streams.push(consumer_stream_config_a.uuid.clone());
    generator_stream_config.output_streams.push(consumer_stream_config_b.uuid.clone());


    let mut engine: YalmEngine = YalmEngine::new();
    engine.add_stream(generator_stream_config);
    engine.add_stream(consumer_stream_config_a);
    engine.add_stream(consumer_stream_config_b);

    if engine.initialise(){
        println!("Engine successfully initialised");
    }

    if engine.start() {
        println!("Engine successfully started");
        thread::sleep(Duration::from_secs(10));
    }
    else {
        println!("Failed to start, validation failed");
    }

}


#[cfg(test)]
mod tests {
}