pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(_input: &str) -> u32 {
    0
}

fn part_two(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(INPUT), 0);
    }

    #[test]
    #[ignore]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 0);
    }
}