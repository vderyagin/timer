use std::io::{Write, stdout};

pub fn update_message(msg: &str) {
  print!("\r");                // move cursor to beginning of the line
  print!("\x1b[K");            // clear line from cursor position to the end
  print!("{}", msg);
  stdout().flush().unwrap();
}

pub fn bell() {
  print!("\x07");
}
