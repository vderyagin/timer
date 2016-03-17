extern crate ansi_term;
extern crate timer;

use ansi_term::Colour::Red;
use std::io::Write;
use std::time::Duration;
use std::{io, thread};
use timer::beeper::Beeper;
use timer::timer::Timer;

fn main() {
  let tmr = Timer::from_args();
  let beeper = Beeper::new();
  let beep_interval = Duration::from_secs(60);

  loop {
    if tmr.is_over() {
      print!("\x07");
      beeper.beep();
    }

    print!("\r{}",
           if tmr.is_over() {
             Red.paint(tmr.status()).to_string()
           } else {
             tmr.status()
           });

    io::stdout().flush().unwrap();
    thread::sleep(beep_interval);
  };
}
