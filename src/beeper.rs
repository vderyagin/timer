use std::io::Cursor;
use rodio;

const SOUND: &'static [u8] = include_bytes!("beep.ogg");

pub struct Beeper {
  sink: rodio::Sink,
}

impl Beeper {
  pub fn new() -> Self {
    let endpoint = rodio::get_default_endpoint().unwrap();
    let mut sink = rodio::Sink::new(&endpoint);
    sink.set_volume(0.3);

    Beeper{sink: sink}
  }

  pub fn beep(&self) {
    self.sink.append(self.decoder());
  }

  fn decoder(&self) -> rodio::Decoder<Cursor<&'static [u8]>> {
    rodio::Decoder::new(Cursor::new(SOUND)).unwrap()
  }
}
