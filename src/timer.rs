use ansi_term::Colour::Red;
use beeper::Beeper;
use options::Options;
use std::io::{Write, stdout};
use std::{thread, time};
use time::{Duration, SteadyTime};

pub struct Timer {
  beeper:  Beeper,
  options: Options,
  start:   SteadyTime,
}

impl Timer {
  fn new(options: Options) -> Self {
    Timer {
      start:   SteadyTime::now(),
      options: options,
      beeper:  Beeper::new(),
    }
  }

  pub fn from_args() -> Self {
    Self::new(Options::new())
  }

  fn is_over(&self, passed: Duration) -> bool {
    passed >= self.options.duration
  }

  fn status(&self, passed: Duration) -> String {
    if self.is_over(passed) {
      Red.paint(format!("{} passed{}",
                        format_duration(passed),
                        self.overtime_string(passed))).to_string()
    } else {
      format!("{} of {} passed{}",
              format_duration(passed),
              format_duration(self.options.duration),
              self.time_left_string(passed))
    }
  }

  fn overtime_string(&self, passed: Duration) -> String {
    let overtime = passed - self.options.duration;
    if overtime.num_minutes() <= 0 { return "".to_string() };
    format!(" ({} overtime)", format_duration(overtime))
  }

  fn time_left_string(&self, passed: Duration) -> String {
    let time_left = self.options.duration - passed;
    if passed.num_minutes() == 0 { return "".to_string() };
    format!(" ({} left)", format_duration(time_left))
  }

  fn maybe_beep(&self, passed: Duration) {
    let overtime_minutes = (passed - self.options.duration).num_minutes();
    let interval_minutes = self.options.beep_interval.num_minutes();
    if self.is_over(passed) && (overtime_minutes % interval_minutes == 0) {
      self.beeper.beep();
      print!("\x07");
    }
  }

  fn update_status_display(&self, passed: Duration) {
    print!("\r");                // return cursor to beginning of the line
    print!("\x1b[K");            // clear line from cursor position to the end
    print!("{}", self.status(passed));
    stdout().flush().unwrap();
  }

  fn sleep_until(&self, time: SteadyTime) {
    let duration = time - SteadyTime::now();
    thread::sleep(time::Duration::from_millis(duration.num_milliseconds() as u64));
  }

  pub fn run(&self) {
    for time_passed in (0..).map(Duration::minutes) {
      self.maybe_beep(time_passed);
      self.update_status_display(time_passed);
      self.sleep_until(self.start + time_passed + Duration::minutes(1));
    }
  }
}

fn format_duration(dur: Duration) -> String {
  format!("{:02}:{:02}", dur.num_hours(), dur.num_minutes() % 60)
}
