use clap::{App, Arg};
use regex::Regex;
use time::Duration;

pub struct Options {
  pub duration:      Duration,
  pub beep_interval: Duration,
}

impl Options {
  pub fn new() -> Self {
    let matches = App::new("timer")
      .version("0.2.1")
      .author("Victor Deryagin <vderyagin@gmail.com>")
      .about("Timer with audio signal")
      .arg(Arg::with_name("DURATION")
           .default_value("00:30")
           .help("Timer duration")
           .validator(validate_time_string))
      .arg(Arg::with_name("beep-interval")
           .takes_value(true)
           .default_value("3")
           .short("b")
           .long("beep-interval")
           .help("Interval between beeps (in minutes) after time runs out")
           .validator(validate_integer))
      .get_matches();

    let duration_spec = matches.value_of("DURATION").unwrap();
    let captures = duration_spec_re().captures(duration_spec).unwrap();

    let hours = captures.name("hours").unwrap_or("0").parse().unwrap();
    let minutes = captures.name("minutes").unwrap().parse().unwrap();

    let beep_interval = matches.value_of("beep-interval").unwrap().parse().unwrap();

    Options {
      duration:      Duration::hours(hours) + Duration::minutes(minutes),
      beep_interval: Duration::minutes(beep_interval),
    }
  }
}

fn duration_spec_re() -> Regex {
  Regex::new(r"^((?P<hours>\d+):)?(?P<minutes>\d+)$").unwrap()
}

fn digits_re() -> Regex {
  Regex::new(r"^\d+$").unwrap()
}

fn validate_time_string(time_string: String) -> Result<(), String> {
  if digits_re().is_match(time_string.as_str()) {
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

fn validate_integer(input: String) -> Result<(), String> {
  if digits_re().is_match(input.as_str()) {
    Ok(())
  } else {
    Err(format!("'{}' is not an integer", input))
  }
}
