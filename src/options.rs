use regex::Regex;
use time::Duration;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
/// Timer with audio signal
pub struct Options {
    /// Timer duration
    #[clap(id = "DURATION", default_value = "00:30", value_parser = parse_duration)]
    pub duration: Duration,

    /// Interval between beeps (in minutes) after time runs out
    #[clap(short, long = "beep-interval", default_value = "3", value_parser = parse_beep_interval)]
    pub beep_interval: Duration,

    /// Don't show any output text (still beeps)
    #[clap(short, long)]
    pub quiet: bool,

    /// Just beep once and exit (to test sound volume)
    #[clap(short, long)]
    pub test: bool,
}

fn parse_duration(input: &str) -> Result<Duration, String> {
    let re = Regex::new(r"^((?P<hours>\d+):)?(?P<minutes>\d+)$").unwrap();

    let captures = re.captures(input).unwrap();

    let hours = captures
        .name("hours")
        .map(|m| m.as_str())
        .unwrap_or("0")
        .parse()
        .unwrap();

    let minutes = captures
        .name("minutes")
        .map(|m| m.as_str())
        .unwrap()
        .parse()
        .unwrap();

    Ok(Duration::hours(hours) + Duration::minutes(minutes))
}

fn parse_beep_interval(input: &str) -> Result<Duration, String> {
    Ok(Duration::minutes(input.parse().unwrap()))
}

impl Default for Options {
    fn default() -> Options {
        Options::parse()
    }
}

// fn digits_re() -> Regex {
//     Regex::new(r"^\d+$").unwrap()
// }

// fn validate_time_string(time_string: String) -> Result<(), String> {
//     if digits_re().is_match(time_string.as_str()) {
//         Ok(())
//     } else if duration_spec_re().is_match(time_string.as_str()) {
//         let minutes = duration_spec_re()
//             .captures(time_string.as_str())
//             .unwrap()
//             .name("minutes")
//             .map(|m| m.as_str())
//             .unwrap()
//             .parse::<usize>()
//             .unwrap();

//         if minutes > 59 {
//             Err("You can't specify more then 59 minutes in hh:mm format".to_string())
//         } else {
//             Ok(())
//         }
//     } else {
//         Err(format!(
//             "'{}' is not a valid duration, use hh:mm or mm format",
//             time_string
//         ))
//     }
// }

// fn validate_integer(input: String) -> Result<(), String> {
//     if digits_re().is_match(input.as_str()) {
//         Ok(())
//     } else {
//         Err(format!("'{}' is not an integer", input))
//     }
// }
