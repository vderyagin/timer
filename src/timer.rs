use ansi_term::Colour::Red;
use beeper::Beeper;
use options::Options;
use std::io::{Write, stdout};
use std::thread::sleep;
use std::time;
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
    self.elapsed_time() >= self.duration
  }

  fn status(&self) -> String {
    if self.is_over() {
      Red.paint(format!("{} passed{}",
                        format_duration(self.elapsed_time()),
                        self.overtime_string())).to_string()
    } else {
      format!("{} of {} passed{}",
              format_duration(self.elapsed_time()),
              format_duration(self.duration),
              self.time_left_string())
    }
  }

  fn overtime_string(&self) -> String {
    if self.overtime().num_minutes() <= 0 { return "".to_string() };
    format!(" ({} overtime)", format_duration(self.overtime()))
  }

  fn time_left_string(&self) -> String {
    if self.elapsed_time().num_minutes() == 0 { return "".to_string() };
    format!(" ({} left)", format_duration(self.time_left()))
  }

  fn elapsed_time(&self) -> Duration {
    SteadyTime::now() - self.start
  }

  fn time_left(&self) -> Duration {
    // 50 milliseconds is to account for delay between now() calls
    self.duration - self.elapsed_time() + Duration::milliseconds(50)
  }

  fn overtime(&self) -> Duration {
    self.elapsed_time() - self.duration
  }

  fn tick(&self) {
    if self.is_over() {
      self.beeper.beep();
      print!("\x07");                     // beep
    };

    print!("\r");                // return cursor to beginning of the line
    print!("\x1b[K");            // clear line from cursor position to the end

    print!("{}", self.status());
    stdout().flush().unwrap();
  }

  pub fn run(&self) {
    loop {
      self.tick();
      sleep(time::Duration::from_secs(60))
    }
  }
}

fn format_duration(dur: Duration) -> String {
  format!("{:02}:{:02}", dur.num_hours(), dur.num_minutes() % 60)
}
