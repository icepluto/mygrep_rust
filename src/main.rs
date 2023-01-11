use mygrep::Config;
use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("problem error :{} ", err);
        process::exit(1);
    });

    if let Err(e) = mygrep::run(config) {
        eprintln!("Application Error : {} ", e);
        process::exit(1);
    };
}
