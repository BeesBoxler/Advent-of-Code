pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    let mut a = Vec::<i32>::new();
    let mut b = Vec::<i32>::new();

    input.lines().for_each(|line| {
        let entries = line.split_whitespace().collect::<Vec<&str>>();
        a.push(entries[0].parse::<i32>().unwrap());
        b.push(entries[1].parse::<i32>().unwrap());
    });

    a.sort();
    b.sort();

    let mut i = 0;

    a.iter()
        .map(|a| {
            i += 1;

            (a - b[i - 1]).abs()
        })
        .sum::<i32>()
}

fn part_two(input: &str) -> usize {
    let mut a = Vec::<usize>::new();
    let mut b = Vec::<usize>::new();

    input.lines().for_each(|line| {
        let entries = line.split_whitespace().collect::<Vec<&str>>();
        a.push(entries[0].parse().unwrap());
        b.push(entries[1].parse().unwrap());
    });

    a.iter()
        .map(|a| b.iter().filter(|b| b == &a).count() * a)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(&INPUT.to_string()), 11);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(&INPUT.to_string()), 31);
    }
}
