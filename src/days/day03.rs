use std::collections::HashSet;

type Compartment = (HashSet<u8>, HashSet<u8>);

pub fn run(input: String) {
    let input = parse_input(&input);

    println!("The total priority is {}", part_one(&input));
    println!("Total sum of all groups: {}", part_two(&input));
}

fn part_one(input: &Vec<Compartment>) -> u32 {
    let mut sum: u32 = 0;

    for line in input {
        sum += line.0.intersection(&line.1).sum::<u8>() as u32;
    }

    sum
}

fn part_two(input: &Vec<Compartment>) -> u32 {
    let mut sum = 0;
    let mut group = Vec::new();

    for line in input {
        group.push(line.0.union(&line.1).map(|x| *x).collect::<HashSet<u8>>());
        if group.len() == 3 {
            sum += group[0]
                .intersection(&group[1])
                .map(|x| *x)
                .collect::<HashSet<u8>>()
                .intersection(&group[2])
                .map(|x| *x)
                .next()
                .unwrap() as u32;
            group.clear();
        }
    }

    sum
}

fn parse_input(input: &String) -> Vec<Compartment> {
    input
        .split('\n')
        .map(|v| {
            let res = v.split_at(v.len() / 2);
            (
                res.0
                    .chars()
                    .map(|char| ascii_to_value(char as u8))
                    .collect(),
                res.1
                    .chars()
                    .map(|char| ascii_to_value(char as u8))
                    .collect(),
            )
        })
        .collect::<Vec<Compartment>>()
}

fn ascii_to_value(code: u8) -> u8 {
    if code > 96 {
        code - 96
    } else {
        code - 38
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn parser_returns_correct_data() {
        let data = parse_input(&INPUT.to_string());
        assert_eq!(
            data,
            vec!(
                (
                    [36, 20, 49, 16, 23, 22, 7, 18].into(),
                    [3, 6, 8, 19, 39, 16, 32].into()
                ),
                (
                    [38, 34, 10, 44, 33, 26, 17, 30, 40].into(),
                    [18, 32, 38, 45, 39, 19, 6, 52].into()
                ),
                (
                    [17, 13, 18, 42, 48, 26, 4].into(),
                    [7, 28, 22, 42, 49, 23, 46].into()
                ),
                (
                    [23, 52, 17, 22, 8, 39, 38, 34].into(),
                    [46, 43, 2, 45, 32, 28, 10, 22, 14, 3].into()
                ),
                ([36, 44, 7, 20, 33].into(), [20, 46, 52, 43, 3].into()),
                (
                    [26, 36, 19, 42, 29, 33, 52, 18].into(),
                    [19, 13, 30, 16, 38, 39, 23].into()
                ),
            )
        )
    }

    #[test]
    fn part_one_returns_correct_output() {
        let data = parse_input(&INPUT.to_string());
        assert_eq!(part_one(&data), 157);
    }

    #[test]
    fn part_two_returns_correct_output() {
        let data = parse_input(&INPUT.to_string());
        assert_eq!(part_two(&data), 70);
    }

    #[test]
    fn ascii_to_value_returns_correct_output() {
        assert_eq!(ascii_to_value('A' as u8), 27);
        assert_eq!(ascii_to_value('Z' as u8), 52);
        assert_eq!(ascii_to_value('a' as u8), 1);
        assert_eq!(ascii_to_value('z' as u8), 26);
    }
}
