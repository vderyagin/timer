extern crate timer;

use std::thread;
use std::time::Duration;
use timer::timer::Timer;

fn main() {
  let tmr = Timer::from_args();
  let check_interval = Duration::from_secs(60);

  loop {
    tmr.tick();
    thread::sleep(check_interval);
  };
}
