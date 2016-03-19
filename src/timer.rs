use ansi_term::Colour::Red;
use beeper::Beeper;
use options::Options;
use std::io::{Write, stdout};
use time::{Duration, SteadyTime};

pub struct Timer {
  start:    SteadyTime,
  duration: Duration,
  beeper:   Beeper,
}

impl Timer {
  fn new(minutes: i64) -> Self {
    Timer {
      start:    SteadyTime::now(),
      duration: Duration::minutes(minutes),
      beeper:   Beeper::new(),
    }
  }

  pub fn from_args() -> Self {
    Self::new(Options::new().duration.num_minutes())
  }

  fn is_over(&self) -> bool {
    (SteadyTime::now() - self.start) >= self.duration
  }

  fn status(&self) -> String {
    if self.is_over() {
      Red.paint(format!("\x07{} passed ({} overtime)",
                        format_duration(self.elapsed_time()),
                        format_duration(self.overtime()))).to_string()
    } else {
      format!("{} of {} passed",
              format_duration(self.elapsed_time()),
              format_duration(self.duration))
    }
  }

  fn elapsed_time(&self) -> Duration {
    SteadyTime::now() - self.start
  }

  fn overtime(&self) -> Duration {
    SteadyTime::now() - (self.start + self.duration)
  }

  pub fn tick(&self) {
    if self.is_over() { self.beeper.beep() };
    print!("\r{}", self.status());
    stdout().flush().unwrap();
  }
}

fn format_duration(dur: Duration) -> String {
  format!("{:02}:{:02}", dur.num_hours(), dur.num_minutes() % 60)
}
