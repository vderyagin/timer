use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};
use std::time::Duration;

pub fn beep() {
    let source = SineWave::new(440).take_duration(Duration::from_secs_f32(0.4)).amplify(0.50);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}
