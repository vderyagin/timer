use ansi_term::{Colour, Style};
use terminal;
use time::Duration;

pub struct TimerMessageFormatter {
    total: Duration,
}

impl TimerMessageFormatter {
    pub fn new(total: Duration) -> Self {
        TimerMessageFormatter { total: total }
    }

    fn output_style(&self, passed: Duration) -> Style {
        if passed >= self.total {
            Colour::Red.normal()
        } else {
            Style::new()
        }
    }

    fn message_after(&self, passed: Duration) -> String {
        let overtime = passed - self.total;
        let mut msg = String::from(format_duration(passed));

        if passed < self.total {
            msg.push_str(format!(" of {}", format_duration(self.total)).as_str());
        }

        msg.push_str(" passed");

        if (passed.whole_minutes() > 0) && (overtime.whole_minutes() < 0) {
            msg.push_str(format!(" ({} left)", format_duration(-overtime)).as_str());
        }

        if overtime.whole_minutes() > 0 {
            msg.push_str(format!(" ({} overtime)", format_duration(overtime)).as_str());
        }

        msg
    }

    pub fn print_message_after(&self, passed: Duration) {
        let msg = self
            .output_style(passed)
            .paint(self.message_after(passed))
            .to_string();
        terminal::update_message(&msg);
    }
}

fn format_duration(dur: Duration) -> String {
    format!("{:02}:{:02}", dur.whole_hours(), dur.whole_minutes() % 60)
}

#[cfg(test)]
mod tests {
    use super::TimerMessageFormatter;
    use time::Duration;

    fn msg(minutes_passed: i64, minutes_total: i64) -> String {
        TimerMessageFormatter::new(Duration::minutes(minutes_total))
            .message_after(Duration::minutes(minutes_passed))
    }

    #[test]
    fn right_after_start() {
        assert_eq!(msg(0, 5), "00:00 of 00:05 passed");
    }

    #[test]
    fn after_some_time_passed() {
        assert_eq!(msg(3, 5), "00:03 of 00:05 passed (00:02 left)");
    }

    #[test]
    fn when_timer_is_over() {
        assert_eq!(msg(5, 5), "00:05 passed");
    }

    #[test]
    fn with_some_overtime() {
        assert_eq!(msg(8, 5), "00:08 passed (00:03 overtime)");
    }
}
