#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

use std::os::consts::SYSNAME;
use std::io::IoResult;


use conn::connect;
use auth::get_auth;

mod conn;
mod auth;

fn main() {
    let re = match SYSNAME {
        "macos" => {
            regex!(r"^([-:a-zA-Z0-9._/]*):([0-9]+)(\.([0-9]+))?$")
        },
        _ => {
            regex!(r"^([-:a-zA-Z0-9._]*):([0-9]+)(\.([0-9]+))?$")
        }
    };

    let connection = connect(re);
    let (auth_name, auth_data) = get_auth();
    println!("{}, {}", auth_name, auth_data);
}
