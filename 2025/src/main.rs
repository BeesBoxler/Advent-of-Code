mod days;
pub mod utils;
use std::{
    env, fs,
    io::{self},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input;

    let day = if let Some(day) = args.get(1) {
        day.parse::<u8>().ok()
    } else {
        None
    };

    if let Some(path) = args.get(2) {
        println!("Loading input file `{:?}`...", fs::canonicalize(path));
        input = match fs::read_to_string(path) {
            Ok(i) => i,
            _ => {
                println!("Invalid file!\nExiting...");
                return;
            }
        };
    } else {
        let stdin = io::stdin().lock();
        match io::read_to_string(stdin) {
            Ok(s) => {
                input = s;
            }
            _ => {
                println!("Invalid input!\nExiting...");
                return;
            }
        }
    }

    days::run_day(input, day);
}
