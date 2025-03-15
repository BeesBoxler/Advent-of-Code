use std::collections::HashMap;

pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn parse_input(input: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let (rules, sequences) = input.split_once("\n\n").unwrap();
    let mut ruleset: HashMap<usize, Vec<usize>> = HashMap::new();

    rules.lines().for_each(|line| {
        let (target, dependent) = line.split_once('|').unwrap();
        let rule = ruleset.entry(target.parse().unwrap()).or_default();
        rule.push(dependent.parse().unwrap());
    });

    let sequences = sequences
        .lines()
        .map(|line| line.split(',').map(|item| item.parse().unwrap()).collect())
        .collect();

    (ruleset, sequences)
}

fn part_one(input: &str) -> usize {
    let (ruleset, sequences) = parse_input(input);
    let mut count = 0;

    sequences.iter().for_each(|sequence| {
        let mut failure = false;

        for (i, value) in sequence.iter().enumerate() {
            for test in &sequence[0..=i] {
                if test == value {
                    continue;
                }

                if ruleset.get(value).unwrap_or(&Vec::new()).contains(test) {
                    failure = true;
                }
            }
        }

        if !failure {
            count += sequence[sequence.len() / 2];
        }
    });

    count
}

fn part_two(input: &str) -> usize {
    let (ruleset, sequences) = parse_input(input);

    sequences
        .iter()
        .map(|sequence| {
            let mut failed = false;
            let mut sequence = sequence.clone();
            for i in 0..sequence.len() {
                for j in 0..i {
                    if let Some(rules) = ruleset.get(&sequence[i]) {
                        if rules.contains(&sequence[j]) {
                            // do the old switcheroo
                            sequence.swap(i, j);
                            failed = true;
                        }
                    }
                }
            }

            if failed {
                dbg!(&sequence);

                return dbg!(sequence[sequence.len() / 2]);
            }

            0
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(INPUT), 143);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 123);
    }
}
