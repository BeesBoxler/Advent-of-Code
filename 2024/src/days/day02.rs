pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> u32 {
    let mut safe = 0;

    input.lines().for_each(|line| {
        if is_valid(line) {
            safe += 1;
        }
    });

    safe
}

fn part_two(input: &str) -> u32 {
    let mut safe = 0;

    input.lines().for_each(|line| {
        if is_valid(line) {
            safe += 1;
        } else {
            let line: Vec<&str> = line.split_whitespace().collect();
            for i in 0..line.len() {
                let line = [&line[..i], &line[i + 1..]].concat().join(" ");

                if is_valid(&line) {
                    safe += 1;

                    break;
                };
            }
        }
    });

    safe
}

fn is_valid(line: &str) -> bool {
    let mut increasing = false;
    let mut decreasing = false;
    let mut safe_amount = true;

    line.split_whitespace()
        .collect::<Vec<&str>>()
        .windows(2)
        .for_each(|items| {
            let a: usize = items[0].parse().unwrap();
            let b: usize = items[1].parse().unwrap();
            let diff = a.abs_diff(b);

            increasing |= a > b;
            decreasing |= a < b;
            safe_amount &= diff <= 3 && diff > 0;
        });

    (increasing ^ decreasing) && safe_amount
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(INPUT), 2);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 4);
    }
}
