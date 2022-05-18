extern crate clap;
extern crate clipboard_ext;

use std::io::{self, Read};

use clap::{App, Arg};

use clipboard_ext::prelude::*;
use clipboard_ext::x11_fork::ClipboardContext;

mod lib;

fn main() {
    let matches = App::new("mOTP-rs")
        .version("0.1")
        .author("Paul O. <contact@paulollivier.fr>")
        .about("mOTP tokens manipulation")
        .arg(
            Arg::with_name("SECRET")
                .help("Shared secret to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("PIN")
                .short("p")
                .long("pin")
                .help("Pin code. If not given, will be expected via stdin.")
                .required(false).takes_value(true),
        )
        .arg(Arg::with_name("clipboard")
            .short("c")
            .long("clipboard")
            .help("Copy to clipboard"))
        .get_matches();

    let secret = matches.value_of("SECRET").unwrap();
    let pin = match matches.is_present("PIN") {
        true => String::from(matches.value_of("PIN").unwrap()),
        false => {
            let mut pin = String::new();
            io::stdin()
                .read_line(&mut pin)
                .expect("coult not read anything from stdin");
            pin
        }
    };

    let otp = lib::create_otp(secret, &pin);
    if matches.is_present("clipboard") {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(otp.to_owned()).unwrap();
        print!("OTP copied in clipboard");
    } else {
        print!("{}", otp);
    }
}
