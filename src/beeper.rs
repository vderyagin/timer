use rodio::{Sink, Decoder, get_default_endpoint};
use std::io::Cursor;

const SOUND: &'static [u8] = include_bytes!("beep.ogg");

pub struct Beeper {
  sink: Sink,
}

impl Beeper {
  pub fn new() -> Self {
    let endpoint = get_default_endpoint().unwrap();
    let mut sink = Sink::new(&endpoint);
    sink.set_volume(0.3);

    Beeper{sink: sink}
  }

  pub fn beep(&self) {
    self.sink.append(self.decoder());
  }

  fn decoder(&self) -> Decoder<Cursor<&'static [u8]>> {
    Decoder::new(Cursor::new(SOUND)).unwrap()
  }
}
