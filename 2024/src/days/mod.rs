mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

pub fn run_day(input: String, day: Option<u8>) {
    match day.unwrap_or(6) {
        1 => day01::run(input),
        2 => day02::run(input),
        3 => day03::run(input),
        4 => day04::run(input),
        5 => day05::run(input),
        6 => day06::run(input),
        _ => panic!("We haven't done that day yet!"),
    }
}
