pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

struct Data {
    ranges: Vec<(usize, usize)>,
    ingredients: Vec<usize>,
}

impl Data {
    fn from_input(input: &str) -> Self {
        let (ranges, ingredients) = input.split_once("\n\n").unwrap();
        let ranges = ranges
            .lines()
            .map(|line| {
                let (a, b) = line.split_once('-').unwrap();
                let a = a.parse().unwrap();
                let b = b.parse().unwrap();

                (a, b)
            })
            .collect();
        let ingredients = ingredients
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();

        Data {
            ranges,
            ingredients,
        }
    }
}

fn part_one(input: &str) -> u32 {
    let Data {
        ranges,
        ingredients,
    } = Data::from_input(input);

    let mut count = 0;
    for ingredient in ingredients {
        if ranges
            .iter()
            .any(|(a, b)| ingredient >= *a && ingredient <= *b)
        {
            count += 1
        }
    }

    count
}

fn part_two(input: &str) -> usize {
    let mut ranges = Data::from_input(input).ranges;
    let mut total = 0;

    ranges.sort_by(|(a, _), (b, _)| a.cmp(b));

    let mut max = 0;

    for (a, b) in ranges {
        if a <= max {
            if b > max {
                total += b - max;
                max = b;
            }
        } else {
            total += b - a + 1;
            max = b;
        }
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(INPUT), 3);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 14);
    }
}
