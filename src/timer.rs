use std::convert::TryFrom;
use std::thread;
use std::time::{Duration as StdDuration, Instant};
use time::Duration;

use crate::beeper::beep;
use crate::options::Options;
use crate::terminal;
use crate::timer_message_formatter::TimerMessageFormatter;

pub struct Timer {
    message_formatter: TimerMessageFormatter,
    options: Options,
    start: Instant,
}

impl Timer {
    pub fn from_args() -> Self {
        Self::new(Default::default())
    }

    fn new(options: Options) -> Self {
        Timer {
            message_formatter: TimerMessageFormatter::new(options.duration),
            options,
            start: Instant::now(),
        }
    }

    pub fn run(&self) {
        if self.options.test {
            terminal::bell();
            beep();
            return;
        }

        for time_passed in (0..).map(Duration::minutes) {
            self.maybe_beep(time_passed);
            if !self.options.quiet {
                self.message_formatter.print_message_after(time_passed);
            }
            self.sleep_until(self.start + time_passed + Duration::minutes(1));
        }
    }

    fn maybe_beep(&self, passed: Duration) {
        let overtime_minutes = (passed - self.options.duration).whole_minutes();
        let interval_minutes = self.options.beep_interval.whole_minutes();
        if (overtime_minutes >= 0) && (overtime_minutes % interval_minutes == 0) {
            beep();
            terminal::bell();
        }
    }

    fn sleep_until(&self, time: Instant) {
        let now = Instant::now();
        if time > now {
            thread::sleep(StdDuration::try_from(time - now).expect("Failed to convert duration"));
        }
    }
}
