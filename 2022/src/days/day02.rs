pub fn run(input: String) {
    println!("Total score for part one: {}", part_one(&input));
    println!("Total score for part two: {}", part_two(&input));
}

fn part_one(input: &String) -> u32 {
    let mut total = 0;
    input.split('\n').for_each(|round| match round {
        "A X" => total += 4,
        "B Y" => total += 5,
        "C Z" => total += 6,

        "C X" => total += 7,
        "A Y" => total += 8,
        "B Z" => total += 9,

        "B X" => total += 1,
        "C Y" => total += 2,
        "A Z" => total += 3,

        _ => {}
    });

    total
}

fn part_two(input: &String) -> u32 {
    let mut total = 0;
    input.split('\n').for_each(|round| match round {
        "A X" => total += 3,
        "A Y" => total += 4,
        "A Z" => total += 8,

        "B X" => total += 1,
        "B Y" => total += 5,
        "B Z" => total += 9,

        "C X" => total += 2,
        "C Y" => total += 6,
        "C Z" => total += 7,

        _ => {}
    });

    total
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(&INPUT.to_string()), 15);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(&INPUT.to_string()), 12);
    }
}
