pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| line.bytes().map(|b| (b - 48) as usize).collect())
        .collect()
}

fn part_one(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|bank| {
            let mut max = 0;
            bank.iter().enumerate().for_each(|(i, j1)| {
                for j in (i + 1)..bank.len() {
                    if j1 * 10 + bank[j] > max {
                        max = j1 * 10 + bank[j]
                    }
                }
            });

            max
        })
        .sum()
}

fn part_two(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(INPUT), 357);
    }

    #[test]
    #[ignore]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 3121910778619);
    }
}

