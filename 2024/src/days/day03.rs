use regex::Regex;

pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    let matcher = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();

    let mut values: Vec<(usize, usize)> = vec![];
    for (_, [a, b]) in matcher.captures_iter(input).map(|c| c.extract()) {
        values.push((a.parse().unwrap(), b.parse().unwrap()));
    }

    values.iter().map(|(a, b)| a * b).sum()
}

fn part_two(input: &str) -> usize {
    let matcher = Regex::new(r"mul\((\d*),(\d*)\)(?:.*don't\(\).*do\(\))?").unwrap();

    let mut values: Vec<(usize, usize)> = vec![];
    for (_, [a,b]) in matcher.captures_iter(input).map(|c| c.extract()) {
        values.push((a.parse().unwrap(), b.parse().unwrap()));
    }

    values.iter().map(|(a, b)| a * b).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(&INPUT.to_string()), 161);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(
            part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
