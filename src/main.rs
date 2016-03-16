extern crate timer;

use timer::timer::Timer;
use std::io::Write;
use std::{env, io, process, thread};
use std::time::Duration;


fn main() {
  let args = env::args().collect::<Vec<_>>();
  let minutes;

  match args.len() {
    1 => minutes = 30,
    2 => if let Ok(arg) = args[1].parse::<i64>() {
      minutes = arg;
    } else {
      die("Only accepting numbers");
    },
    _ => die("At most one argument is allowed")
  }

  let tmr = Timer::new(minutes);

  loop {
    if tmr.is_over() {
    }

    print!("\r{}", tmr.status());
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(60));
  };
}

fn die(message: &str) -> ! {
  writeln!(&mut io::stderr(), "{}", message).unwrap();
  process::exit(1);
}
