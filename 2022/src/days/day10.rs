const SCREEN_WIDTH: u8 = 40;
const INTERVALS: [u8; 6] = [20, 60, 100, 140, 180, 220];
use std::io::{stdout, Write};

use crate::utils::wait;

pub fn run(input: String) {
    println!("{}", part_one(&input));
    part_two(&input);
}

fn part_one(input: &String) -> i32 {
    let mut signal_strengths = Vec::new();
    let mut x = 1;
    let mut cycle: i32 = 0;
    let mut lines = input.lines().into_iter();
    let mut buffer: i32 = 0;

    loop {
        cycle += 1;

        if INTERVALS.contains(&(cycle as u8)) {
            signal_strengths.push(x * cycle);
        }

        if buffer != 0 {
            x += buffer;
            buffer = 0;
        } else if let Some(instruction) = lines.next() {
            if let Some((_, value)) = instruction.split_once(' ') {
                buffer += value.parse::<i32>().unwrap();
            }
        } else {
            break;
        }
    }

    signal_strengths.into_iter().sum()
}

fn part_two(input: &String) -> String {
    let mut out = stdout().lock();
    let mut x: i32 = 1;
    let mut cycle: u8 = 0;
    let mut lines = input.lines().into_iter();
    let mut buffer: i32 = 0;
    let mut output = Vec::new();

    loop {
        let position = cycle % SCREEN_WIDTH as u8;
        let char;

        if position as i32 >= x - 1 && position as i32 <= x + 1 {
            char = '#';
        } else {
            char = '.';
        }

        cycle += 1;

        if buffer != 0 {
            x += buffer;
            buffer = 0;
        } else if let Some(instruction) = lines.next() {
            if let Some((_, value)) = instruction.split_once(' ') {
                buffer += value.parse::<i32>().unwrap();
            }
        } else {
            break;
        }

        if !cfg!(test) {
            out.write(&[char as u8]).unwrap();
            wait("0.05");
        }

        output.push(char);

        if position == SCREEN_WIDTH - 1 {
            if !cfg!(test) {
                out.write(&[10]).unwrap();
            }
            output.push('\n');
        }

        out.flush().unwrap();
    }
    output.pop(); // Trailing \n
    output.iter().collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

const OUTPUT: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(&INPUT.to_string()), 13140);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(&INPUT.to_string()), OUTPUT.to_string());
    }
}
