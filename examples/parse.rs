use std::io::prelude::*;
use std::io::{stdin, stdout};

use cozy_uci::command::UciCommand;
use cozy_uci::remark::UciRemark;
use cozy_uci::{UciFormatOptions, UciParseErrorKind};
use UciParseErrorKind::*;

fn main() {
    let options = UciFormatOptions::default();
    loop {
        print!(">");
        stdout().flush().unwrap();
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

        match UciCommand::parse_from(&line, &options) {
            Ok(cmd) => {
                println!("{:?}", cmd);
                continue;
            }
            Err(err) => {
                if !matches!(err.kind, UnknownMessageKind(_)) {
                    println!("{}", err);
                    continue;
                }
            }
        }
        match UciRemark::parse_from(&line, &options) {
            Ok(rmk) => {
                println!("{:?}", rmk);
                continue;
            }
            Err(err) => {
                println!("error: {}", err);
                continue;
            }
        }
    }
}
