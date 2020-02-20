use rodio::{Sink, Decoder, default_output_device};
use std::io::Cursor;

const SOUND: &'static [u8] = include_bytes!("beep.ogg");

pub struct Beeper {
  sink: Sink,
}

impl Beeper {
  pub fn beep(&self) {
    self.sink.append(self.decoder());
  }

  fn decoder(&self) -> Decoder<Cursor<&'static [u8]>> {
    Decoder::new(Cursor::new(SOUND)).unwrap()
  }
}

impl Default for Beeper {
  fn default() -> Beeper {
    let endpoint = default_output_device().unwrap();
    let sink = Sink::new(&endpoint);
    sink.set_volume(0.3);

    Beeper{sink: sink}
  }
}
