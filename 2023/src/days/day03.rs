const OFFSET: u8 = 48;
const PERIOD: u8 = 46;

type NumberList = Vec<Vec<u8>>;

pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    let input = input
        .lines()
        .map(|line| line.bytes().collect())
        .collect::<NumberList>();
    let is_number = |x: &u8| (OFFSET..OFFSET + 10).contains(x);
    let is_special = |x: &u8| !is_number(x) && *x != PERIOD;
    let to_number = |x: &u8| *x - OFFSET;
    let is_valid = |i: usize, j: usize| {
        let mut valid = false;

        if i > 0 {
            valid |= is_special(&input[i - 1][j]);
            if j > 0 {
                valid |= is_special(&input[i - 1][j - 1]);
            }
            if j < input[i].len() - 1 {
                valid |= is_special(&input[i - 1][j + 1]);
            }
        }

        if i < input.len() - 1 {
            valid |= is_special(&input[i + 1][j]);
            if j > 0 {
                valid |= is_special(&input[i + 1][j - 1]);
            }
            if j < &input[i].len() - 1 {
                valid |= is_special(&input[i + 1][j + 1]);
            }
        }

        if j > 0 {
            valid |= is_special(&input[i][j - 1]);
        }
        if j < input[i].len() - 1 {
            valid |= is_special(&input[i][j + 1]);
        }

        valid
    };

    let mut sum: usize = 0;

    for (i, line) in input.iter().enumerate() {
        let mut curr: usize = 0;
        let mut is_curr_valid = false;
        for (j, byte) in line.iter().enumerate() {
            if is_number(byte) {
                is_curr_valid |= is_valid(i, j);
                curr *= 10;
                curr += to_number(byte) as usize;
            }
            if !is_number(byte) || j == line.len() - 1 {
                if is_curr_valid {
                    sum += curr;
                }
                is_curr_valid = false;
                curr = 0;
            }
        }
    }

    sum
}

fn part_two(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(&INPUT.to_string()), 4361);
    }

    #[test]
    #[ignore]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(&INPUT.to_string()), 0);
    }
}
