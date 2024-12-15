mod graph;

use graph::Dag;

pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    let mut sum = 0;
    let (edges, data) = input.split_once("\n\n").unwrap();
    let graph = Dag::create(
        edges
            .lines()
            .map(|line| {
                let (k, v) = line.split_once("|").unwrap();
                (k.parse().unwrap(),v.parse().unwrap())
            })
            .collect(),
    );

    data.lines().for_each(|line| {
        let line = line
            .split(",")
            .map(|digit| digit.parse().unwrap())
            .collect::<Vec<usize>>();
        let valid = line.windows(2).all(|pair| {
            let a = graph.iter().position(|&r| r == pair[0]);
            let b = graph.iter().position(|&r| r == pair[1]);

            a < b

        });

        if valid {
            sum += line[line.len() / 2];
        }
    });

    sum
}

fn part_two(input: &str) -> u32 {
    0
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
        assert_eq!(part_one(&INPUT.to_string()), 143);
    }

    #[test]
    #[ignore]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(&INPUT.to_string()), 0);
    }
}
