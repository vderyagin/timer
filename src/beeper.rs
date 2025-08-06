use rodio::source::{SineWave, Source};
use rodio::{OutputStreamBuilder, Sink};
use std::time::Duration;

const BEEP_FREQUENCY: f32 = 440.0;
const BEEP_DURATION: f32 = 0.4;

pub fn beep() {
    let source = SineWave::new(BEEP_FREQUENCY)
        .take_duration(Duration::from_secs_f32(BEEP_DURATION))
        .amplify(0.50);
    let stream_handle = OutputStreamBuilder::open_default_stream().unwrap();
    let sink = Sink::connect_new(&stream_handle.mixer());

    sink.append(source);
    sink.sleep_until_end();
}
