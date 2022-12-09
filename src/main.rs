mod days;
use std::{fs, env, io::{self}};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input;

    if let Some(path) = args.get(1) {
        println!("Loading input file `{:?}`...", fs::canonicalize(path));
        input = match fs::read_to_string(path) {
            Ok(i) => { i }
            _ => {
                println!("Invalid file!\nExiting...");
                return
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

    
    days::run_day(input);
}