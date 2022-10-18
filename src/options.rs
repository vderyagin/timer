use clap::Parser;
use regex::Regex;
use time::Duration;

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

    let captures = re.captures(input).ok_or(format!(
        "'{}' is not a valid duration, use hh:mm or mm format",
        input
    ))?;

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

    if captures.name("hours").is_some() && minutes > 59 {
        return Err("You can't specify more then 59 minutes in hh:mm format".to_string());
    }

    Ok(Duration::hours(hours) + Duration::minutes(minutes))
}

fn parse_beep_interval(input: &str) -> Result<Duration, String> {
    let minutes = input
        .parse()
        .map_err(|_| format!("'{}' is not a valid integer", input))?;

    if minutes == 0 {
        return Err("beep interval can't be zero".to_string());
    }

    Ok(Duration::minutes(minutes))
}

impl Default for Options {
    fn default() -> Options {
        Options::parse()
    }
}

#[cfg(test)]
mod tests {
    mod parse_beep_interval {
        use super::super::parse_beep_interval;

        #[test]
        fn accepts_integers() {
            assert!(parse_beep_interval("14").is_ok());
            assert!(parse_beep_interval("1").is_ok());
            assert!(parse_beep_interval("333").is_ok());
        }

        #[test]
        fn does_not_accept_zero() {
            assert!(parse_beep_interval("0").is_err());
        }

        #[test]
        fn does_not_accept_any_funny_formats() {
            assert!(parse_beep_interval("3:33").is_err());
        }
    }

    mod parse_duration {
        use super::super::parse_duration;

        #[test]
        fn parses_hh_mm_format() {
            assert!(parse_duration("12:34").is_ok());
            assert!(parse_duration("1:32").is_ok());
            assert!(parse_duration("01:12").is_ok());
        }

        #[test]
        fn parses_mm_format() {
            assert!(parse_duration("1").is_ok());
            assert!(parse_duration("98").is_ok());
            assert!(parse_duration("1123").is_ok());
        }

        #[test]
        fn general_format_validation() {
            assert!(parse_duration("aaa").is_err());
        }

        #[test]
        fn at_most_59_minutes_in_hh_mm_format() {
            assert!(parse_duration("1:59").is_ok());
            assert!(parse_duration("1:60").is_err());
        }

        #[test]
        fn can_have_any_number_of_minutes_in_mm_format() {
            assert!(parse_duration("99999").is_ok());
            assert_eq!(parse_duration("80"), parse_duration("1:20"));
        }

        #[test]
        fn cant_specify_just_hours() {
            assert!(parse_duration("1:").is_err());
            assert!(parse_duration("1:0").is_ok());
        }

        #[test]
        fn leading_zeroes_dont_hurt() {
            assert_eq!(parse_duration("1:2"), parse_duration("00001:0000002"));
        }
    }
}
