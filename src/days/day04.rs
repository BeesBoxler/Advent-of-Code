use std::collections::HashSet;

type Line = Vec<HashSet<u8>>;

pub fn run(input: String) {
    let formatted_input = parse_input(&input);

    println!(
        "Number of sets fully contained by another: {}",
        part_one(&formatted_input)
    );
    println!(
        "Number of sets with overlaps: {}",
        part_two(&formatted_input)
    );
}

fn part_one(input: &Vec<Line>) -> u32 {
    let mut sum = 0;
    for line in input {
        let union_length = line[0].union(&line[1]).count();
        if union_length == line[0].len() || union_length == line[1].len() {
            sum += 1;
        }
    }

    sum
}

fn part_two(input: &Vec<Line>) -> u32 {
    let mut sum = 0;
    for line in input {
        if line[0].intersection(&line[1]).count() > 0 {
            sum += 1;
        }
    }

    sum
}

fn parse_input(input: &String) -> Vec<Line> {
    input
        .split('\n')
        .map(|line| {
            line.split(',')
                .map(|range| {
                    let bounds = range
                        .split('-')
                        .map(|i| i.parse().unwrap())
                        .collect::<Vec<u8>>();
                    (bounds[0]..=bounds[1]).collect()
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn parser_returns_correct_output() {
        let data = parse_input(&INPUT.to_string());
        assert_eq!(
            data,
            vec!(
                [[4, 3, 2].into(), [6, 8, 7].into()],
                [[3, 2].into(), [5, 4].into()],
                [[7, 5, 6].into(), [7, 8, 9].into()],
                [[2, 8, 5, 6, 7, 3, 4].into(), [5, 6, 3, 4, 7].into()],
                [[6].into(), [4, 6, 5].into()],
                [[2, 3, 4, 5, 6].into(), [7, 8, 4, 5, 6].into()]
            )
        )
    }

    #[test]
    fn part_one_returns_correct_output() {
        let data = parse_input(&INPUT.to_string());
        assert_eq!(part_one(&data), 2);
    }

    #[test]
    fn part_two_returns_correct_output() {
        let data = parse_input(&INPUT.to_string());
        assert_eq!(part_two(&data), 4);
    }
}
