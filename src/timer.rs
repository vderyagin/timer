use std::thread;
use time::{Duration, SteadyTime};

use beeper::Beeper;
use options::Options;
use timer_message_formatter::TimerMessageFormatter;

pub struct Timer {
  beeper:            Beeper,
  message_formatter: TimerMessageFormatter,
  options:           Options,
  start:             SteadyTime,
}

impl Timer {
  pub fn from_args() -> Self {
    Self::new(Options::new())
  }

  fn new(options: Options) -> Self {
    Timer {
      beeper:            Beeper::new(),
      message_formatter: TimerMessageFormatter::new(options.duration),
      options:           options,
      start:             SteadyTime::now(),
    }
  }

  pub fn run(&self) {
    for time_passed in (0..).map(Duration::minutes) {
      self.maybe_beep(time_passed);
      self.message_formatter.print_message_after(time_passed);
      self.sleep_until(self.start + time_passed + Duration::minutes(1));
    }
  }

  fn maybe_beep(&self, passed: Duration) {
    let overtime_minutes = (passed - self.options.duration).num_minutes();
    let interval_minutes = self.options.beep_interval.num_minutes();
    if (overtime_minutes >= 0) && (overtime_minutes % interval_minutes == 0) {
      self.beeper.beep();
      print!("\x07");
    }
  }

  fn sleep_until(&self, time: SteadyTime) {
    thread::sleep((time - SteadyTime::now()).to_std().unwrap())
  }
}
