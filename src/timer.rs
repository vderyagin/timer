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
    format!("{} of {} minute(s) passed",
            time_elapsed.num_minutes(),
            self.duration.num_minutes())
  }
}
