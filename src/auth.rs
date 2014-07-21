use std::os;
use std::io::File;

pub fn get_auth() -> (String, Vec<u8>) {
    let mut auth_name = String::new();
    let mut auth_data = Vec::new();

    let filename = match os::getenv("XAUTHORITY") {
        Some(filename) => filename,
        None => {
            match os::getenv("HOME") {
                Some(path) => format!("{}/.Xauthority", path),
                None => fail!("Xauthority file not found")
            }
        }
    };
    let filepath = Path::new(filename);
    let mut auth_file = match File::open(&filepath) {
        Ok(file) => file,
        Err(err) => fail!("Error opening file: {}", err)
    };

    loop {
        let family = match auth_file.read_be_u16() {
            Ok(family) => family.to_uint().unwrap(),
            // Err(err) => fail!("Failed while parsing family: {}", err)
            Err(err) => break
        };

        let addr_len = match auth_file.read_be_u16() {
            Ok(len) => len.to_uint().unwrap(),
            Err(err) => fail!(err)
        };
        let addr_chars = match auth_file.read_exact(addr_len) {
            Ok(chars) => chars,
            Err(err) => fail!(err)
        };

        let display_no_len = match auth_file.read_be_u16() {
            Ok(len) => len.to_uint().unwrap(),
            Err(err) => fail!(err)
        };
        let display_no_chars = match auth_file.read_exact(display_no_len) {
            Ok(chars) => chars,
            Err(err) => fail!(err)
        };

        let name_len = match auth_file.read_be_u16() {
            Ok(len) => len.to_uint().unwrap(),
            Err(err) => fail!(err)
        };
        let name_chars = match auth_file.read_exact(name_len) {
            Ok(chars) => chars,
            Err(err) => fail!(err)
        };

        let data_len = match auth_file.read_be_u16() {
            Ok(len) => len.to_uint().unwrap(),
            Err(err) => fail!(err)
        };
        let data_chars = match auth_file.read_exact(data_len) {
            Ok(chars) => chars,
            Err(err) => fail!(err)
        };

        let addr = String::from_utf8_lossy(addr_chars.as_slice()).into_string();
        let name = String::from_utf8_lossy(name_chars.as_slice()).into_string();

        if family == 256 && addr == "OD-2.local".to_string() {
            auth_name = name;
            auth_data = data_chars;
        }

        if auth_file.eof() {
            break;
        }
    }

    return (auth_name, auth_data)
}
