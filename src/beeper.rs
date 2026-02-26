use rodio::source::{SineWave, Source};
use rodio::{DeviceSinkBuilder, Player};
use std::time::Duration;

const BEEP_FREQUENCY: f32 = 440.0;
const BEEP_DURATION: f32 = 0.4;

pub fn beep() {
    let source = SineWave::new(BEEP_FREQUENCY)
        .take_duration(Duration::from_secs_f32(BEEP_DURATION))
        .amplify(0.50);
    let mut device_sink =
        DeviceSinkBuilder::open_default_sink().expect("Failed to open audio output stream");
    device_sink.log_on_drop(false);
    let player = Player::connect_new(&device_sink.mixer());

    player.append(source);
    player.sleep_until_end();
}
