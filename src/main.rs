use std::env;
use std::process;

#[allow(non_snake_case)]
use aoc_2021_4::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });    

    if let Err(e) = aoc_2021_4::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}