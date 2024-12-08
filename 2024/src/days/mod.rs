mod day01;
mod day02;
mod day03;
mod day01;

pub fn run_day(input: String, day: Option<u8>) {
    match day.unwrap_or(1) {
        1 => day01::run(input),
        2 => day02::run(input),
        3 => day03::run(input),
        1 => day01::run(input),
        _ => panic!("We haven't done that day yet!"),
    }
}
