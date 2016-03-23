extern crate timer;

use std::thread;
use std::time::Duration;
use timer::timer::Timer;

fn main() {
  let timer = Timer::from_args();
  let check_interval = Duration::from_secs(60);

  loop {
    timer.tick();
    thread::sleep(check_interval);
  };
}
