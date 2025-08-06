use std::io::{stdout, Write};

const CURSOR_TO_START: &str = "\r";
const CLEAR_LINE: &str = "\x1b[K";
const BELL: &str = "\x07";

pub fn update_message(msg: &str) {
    print!("{}", CURSOR_TO_START); // move cursor to beginning of the line
    print!("{}", CLEAR_LINE); // clear line from cursor position to the end
    print!("{}", msg);
    stdout().flush().unwrap();
}

pub fn bell() {
    print!("{}", BELL);
}
