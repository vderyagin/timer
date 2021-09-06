## About

CLI timer. Beeps after set period of time passes, beeps periodically (every 3 minutes, customizable) after that until you ^C it.

## Installation
```sh
cargo install --git https://github.com/vderyagin/timer
```

## Usage
```
Timer with audio signal

USAGE:
    timer [FLAGS] [OPTIONS] [DURATION]

FLAGS:
    -h, --help       Prints help information
    -q, --quiet      Don't show any output text (still beeps)
    -t, --test       Just beep once and exit (to test sound volume)
    -V, --version    Prints version information

OPTIONS:
    -b, --beep-interval <beep-interval>    Interval between beeps (in minutes) after time runs out [default: 3]

ARGS:
    <DURATION>    Timer duration [default: 00:30]
```
