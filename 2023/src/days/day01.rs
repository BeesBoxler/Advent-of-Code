pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &String) -> u32 {
    input
        .lines()
        .map(|line| {
            let numbers = line
                .bytes()
                .filter(|byte| byte > &47u8 && byte < &58u8)
                .collect::<Vec<u8>>();

            (numbers[0] as u32 - 48) * 10 + (numbers[numbers.len() - 1] as u32 - 48)
        })
        .sum()
}

fn part_two(input: &String) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(&INPUT.to_string()), 142);
    }

    #[test]
    #[ignore]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(&INPUT.to_string()), 0);
    }
}
