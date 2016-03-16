use ansi_term::Colour::Red;
use std::string::ToString;
use time::{Duration, SteadyTime};

pub struct Timer {
  start:    SteadyTime,
  duration: Duration,
}

impl Timer {
  pub fn new(minutes: i64) -> Self {
    Timer{
      start:    SteadyTime::now(),
      duration: Duration::minutes(minutes),
    }
  }

  pub fn is_over(&self) -> bool {
    (SteadyTime::now() - self.start) >= self.duration
  }

  pub fn status(&self) -> String {
    let time_elapsed = SteadyTime::now() - self.start;
    let msg = format!("{} of {} minute(s) passed",
                      time_elapsed.num_minutes(),
                      self.duration.num_minutes());

    if self.is_over() {
      Red.paint(msg).to_string()
    } else {
      msg
    }
  }
}
