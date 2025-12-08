use fancy_regex::Regex;
type Number = usize;

pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn parse_input(input: &str) -> Vec<Vec<Number>> {
    input
        .trim()
        .split(',')
        .map(|pair| {
            let (a, b) = pair.split_once('-').unwrap();
            let a = a.parse::<Number>().unwrap();
            let b = b.parse::<Number>().unwrap();

            (a..=b).collect()
        })
        .collect()
}

fn part_one(input: &str) -> Number {
    let parsed_input = parse_input(input);
    let values = parsed_input.iter().flatten().collect::<Vec<_>>();
    let rx = Regex::new(r"^(\d+)\1{1}$").unwrap();

    values
        .iter()
        .filter(|input| {
            let chars = input.to_string();
            let matches = rx.find(&chars).expect("oh no");

            matches.is_some()
        })
        .map(|n| **n)
        .sum()
}

fn part_two(input: &str) -> Number {
    let parsed_input = parse_input(input);
    let values = parsed_input.iter().flatten().collect::<Vec<_>>();
    let rx = Regex::new(r"^(\d+)\1+$").unwrap();

    values
        .iter()
        .filter(|input| {
            let chars = input.to_string();
            let matches = rx.find(&chars).expect("oh no");

            matches.is_some()
        })
        .map(|n| **n)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(INPUT), 1227775554);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 4174379265);
    }
}
