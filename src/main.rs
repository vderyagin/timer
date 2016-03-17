extern crate ansi_term;
extern crate clap;
extern crate regex;
extern crate timer;

use std::io::Write;
use std::time::Duration;
use std::{io, thread};

use timer::beeper::Beeper;
use timer::timer::Timer;

use ansi_term::Colour::Red;
use clap::{App, Arg};
use regex::Regex;

fn main() {
  let matches = App::new("rnm")
    .version("0.1.0")
    .author("Victor Deryagin <vderyagin@gmail.com>")
    .about("Timer with audio signal")
    .arg(Arg::with_name("DURATION")
         .default_value("00:30")
         .help("Timer duration")
         .validator(validate_time_string))
    .get_matches();

  let duration_spec = matches.value_of("DURATION").unwrap();
  let captures = duration_spec_re().captures(duration_spec).unwrap();

  let hours = captures.name("hours").unwrap_or("0").parse::<i64>().unwrap();
  let minutes = captures.name("minutes").unwrap().parse::<i64>().unwrap();

  let tmr = Timer::new(hours * 60 + minutes);
  let beeper = Beeper::new();
  let beep_interval = Duration::from_secs(60);

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
    thread::sleep(beep_interval);
  };
}

fn duration_spec_re() -> Regex {
  Regex::new(r"^((?P<hours>\d+):)?(?P<minutes>\d+)$").unwrap()
}

fn validate_time_string(time_string: String) -> Result<(), String> {
  if Regex::new(r"^\d+$").unwrap().is_match(time_string.as_str()) {
    Ok(())
  } else if duration_spec_re().is_match(time_string.as_str()) {
    let minutes = duration_spec_re()
      .captures(time_string.as_str()).unwrap()
      .name("minutes").unwrap()
      .parse::<usize>().unwrap();

    if minutes > 59 {
      Err("You can't specify more then 59 minutes in hh:mm format".to_string())
    } else {
      Ok(())
    }
  } else {
    Err(format!("'{}' is not a valid duration, use hh:mm or mm format", time_string))
  }
}
