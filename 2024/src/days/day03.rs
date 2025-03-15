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
    let mut sum = 0;
    input.split("do()").for_each(|chunk| {
        sum += part_one(chunk.split_once("don't()").unwrap_or((chunk, "")).0);
    });

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(INPUT), 161);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 48);
    }
}
