use clap::{App, Arg};
use regex::Regex;
use time::Duration;

pub struct Options {
  pub duration: Duration,
}

impl Options {
  pub fn new() -> Self {
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

    let hours = captures.name("hours").unwrap_or("0").parse().unwrap();
    let minutes = captures.name("minutes").unwrap().parse().unwrap();

    Options {
      duration: Duration::hours(hours) + Duration::minutes(minutes)
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
