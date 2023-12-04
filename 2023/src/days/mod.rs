mod day01;
mod day02;

pub fn run_day(input: String, day: Option<u8>) {
    match day.unwrap_or(2) {
        1 => day01::run(input),
        2 => day02::run(input),
        _ => panic!("We haven't done that day yet!"),
    }
}
