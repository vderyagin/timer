use ansi_term::Colour::Red;
use beeper::Beeper;
use options::Options;
use std::io::{Write, stdout};
use std::{thread, time};
use time::{Duration, SteadyTime};

pub struct Timer {
  start:    SteadyTime,
  duration: Duration,
  passed:   Duration,
  beeper:   Beeper,
}

impl Timer {
  fn new(minutes: i64) -> Self {
    Timer {
      start:    SteadyTime::now(),
      duration: Duration::minutes(minutes),
      passed:   Duration::minutes(0),
      beeper:   Beeper::new(),
    }
  }

  pub fn from_args() -> Self {
    Self::new(Options::new().duration.num_minutes())
  }

  fn is_over(&self) -> bool {
    self.passed >= self.duration
  }

  fn status(&self) -> String {
    if self.is_over() {
      Red.paint(format!("{} passed{}",
                        format_duration(self.passed),
                        self.overtime_string())).to_string()
    } else {
      format!("{} of {} passed{}",
              format_duration(self.passed),
              format_duration(self.duration),
              self.time_left_string())
    }
  }

  fn overtime_string(&self) -> String {
    let overtime = self.passed - self.duration;
    if overtime.num_minutes() <= 0 { return "".to_string() };
    format!(" ({} overtime)", format_duration(overtime))
  }

  fn time_left_string(&self) -> String {
    if self.passed.num_minutes() == 0 { return "".to_string() };
    format!(" ({} left)", format_duration(self.duration - self.passed))
  }

  fn tick(&mut self) {
    if self.is_over() {
      self.beeper.beep();
      print!("\x07");                     // beep
    };

    print!("\r");                // return cursor to beginning of the line
    print!("\x1b[K");            // clear line from cursor position to the end

    print!("{}", self.status());
    stdout().flush().unwrap();
  }

  fn sleep(&self) {
    let dur = self.start + self.passed + Duration::minutes(1) - SteadyTime::now();
    thread::sleep(time::Duration::from_millis(dur.num_milliseconds() as u64));
  }

  pub fn run(&mut self) {
    loop {
      self.tick();
      self.sleep();
      self.passed = self.passed + Duration::minutes(1);
    }
  }
}

fn format_duration(dur: Duration) -> String {
  format!("{:02}:{:02}", dur.num_hours(), dur.num_minutes() % 60)
}
