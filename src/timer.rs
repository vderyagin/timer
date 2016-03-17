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
    format!("{}/{} passed",
            format_duration(SteadyTime::now() - self.start),
            format_duration(self.duration))
  }
}

fn format_duration(dur: Duration) -> String {
  format!("{:02}:{:02}", dur.num_hours(), dur.num_minutes() % 60)
}
