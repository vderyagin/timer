extern crate timer;
extern crate ansi_term;

use std::io::Write;
use std::time::Duration;
use std::{env, io, process, thread};
use ansi_term::Colour::Red;

use timer::beeper::Beeper;
use timer::timer::Timer;

fn main() {
  let args = env::args().collect::<Vec<_>>();
  let minutes;

  match args.len() {
    1 => minutes = 30,
    2 => if let Ok(arg) = args[1].parse::<i64>() {
      minutes = arg;
    } else {
      die("Only accepting numbers");
    },
    _ => die("At most one argument is allowed")
  }

  let tmr = Timer::new(minutes);
  let beeper = Beeper::new();

  loop {
    if tmr.is_over() {
      beeper.beep();
    }

    print!("\r{}",
           if tmr.is_over() {
             Red.paint(tmr.status()).to_string()
           } else {
             tmr.status()
           });

    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(60));
  };
}

fn die(message: &str) -> ! {
  writeln!(&mut io::stderr(), "{}", message).unwrap();
  process::exit(1);
}
