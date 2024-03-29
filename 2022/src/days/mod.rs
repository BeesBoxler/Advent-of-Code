mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

pub fn run_day(input: String, day: Option<u8>) {
    match day.unwrap_or(12) {
        1 => day01::run(input),
        2 => day02::run(input),
        3 => day03::run(input),
        4 => day04::run(input),
        5 => day05::run(input),
        6 => day06::run(input),
        7 => day07::run(input),
        8 => day08::run(input),
        9 => day09::run(input),
        10 => day10::run(input),
        11 => day11::run(input),
        12 => day12::run(input),
        _ => panic!("We haven't done that day yet!"),
    }
}
