use lib::{stream::{dummy_stream::DummyStreamConfig, Direction, StreamConfig, StreamTypeConfig}, yalm_config::YalmConfig, yalm_engine::YalmEngine};



fn main() {
    let dummy_stream_config_gen: DummyStreamConfig = DummyStreamConfig {message_generation_rate_hz: 1};
    let mut generator_stream_config: StreamConfig = StreamConfig::default();
    generator_stream_config.name = String::from("Generator Stream");
    generator_stream_config.direction = Direction::Output;
    generator_stream_config.type_config = StreamTypeConfig::Dummy { config:dummy_stream_config_gen };

    let dummy_stream_config_con: DummyStreamConfig = DummyStreamConfig {message_generation_rate_hz: 1};
    let mut consumer_stream_config_a: StreamConfig = StreamConfig::default();
    consumer_stream_config_a.name = String::from("Consumer Stream A");
    consumer_stream_config_a.direction = Direction::Input;
    consumer_stream_config_a.type_config = StreamTypeConfig::Dummy { config:dummy_stream_config_con };
    consumer_stream_config_a.input_filter = String::from("tick");

    let dummy_stream_config_con: DummyStreamConfig = DummyStreamConfig {message_generation_rate_hz: 1};
    let mut consumer_stream_config_b: StreamConfig = StreamConfig::default();
    consumer_stream_config_b.name = String::from("Consumer Stream A");
    consumer_stream_config_b.direction = Direction::Input;
    consumer_stream_config_b.type_config = StreamTypeConfig::Dummy { config:dummy_stream_config_con };
    consumer_stream_config_b.input_filter = String::from("tock");

    generator_stream_config.output_streams.push(consumer_stream_config_a.uuid.clone());
    generator_stream_config.output_streams.push(consumer_stream_config_b.uuid.clone());


    let mut engine: YalmEngine = YalmEngine::new();
    engine.add_stream(generator_stream_config);
    engine.add_stream(consumer_stream_config_a);
    engine.add_stream(consumer_stream_config_b);

    engine.start();

}


#[cfg(test)]
mod tests {
}