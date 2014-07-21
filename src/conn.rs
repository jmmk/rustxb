use std::io::net::unix::UnixStream;
use std::os;
use regex::Regex;

pub fn connect(re: Regex) -> UnixStream {
    let display = match os::getenv("DISPLAY") {
        Some(display) => display,
        None => fail!("DISPLAY environment variable is empty")
    };
    let captures = match re.captures(display.as_slice()) {
        Some(captures) => captures,
        None => fail!("Invalid display string")
    };

    let host = captures.at(1).to_string();
    let display_no: u8 = from_str(captures.at(2)).unwrap();
    let screen_no: u8 = match from_str(captures.at(3)) {
        Some(number) => number,
        None => 0
    };

    let socket = Path::new(display.as_slice());
    let mut connection = match UnixStream::connect(&socket) {
        Ok(conn) => conn,
        Err(err) => fail!("Error opening connection: {}", err)
    };

    return connection
}
