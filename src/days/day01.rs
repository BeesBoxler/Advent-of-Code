type Number = u32;

pub fn run(input: String) {
    let elf_totals = parse_input(input);

    println!("Elf carrying most calories is carrying {}", part_one(&elf_totals));
    println!("3 Elves carrying the most calories are carrying {}", part_two(&elf_totals));
}

fn part_one(elf_totals: &Vec<Number>) -> Number {
    let mut sorted_elves = elf_totals.clone();
    sorted_elves.sort_by(|a,b| b.cmp(a));

    sorted_elves[0]
}

fn part_two(elf_totals: &Vec<Number>) -> Number {
    let mut sorted_elves = elf_totals.clone();
    sorted_elves.sort_by(|a,b| b.cmp(a));

    sorted_elves[..3].iter().sum()
}

fn parse_input(input: String) -> Vec<Number> {
    let mut elf_totals = vec![0];

    input.split("\n").for_each(|v| {

        if let Ok(value) = v.parse::<Number>() {
            let index = elf_totals.len() - 1;
            elf_totals[index] += value;
        } else {
            elf_totals.push(0);
        }
    } );

    return elf_totals;
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: & str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn parser_returns_correct_output() {
        let data = parse_input(INPUT.to_string());
        assert_eq!(data, vec!(6000, 4000, 11000, 24000, 10000));
    }

    #[test]
    fn part_one_returns_correct_output() {
        let data = parse_input(INPUT.to_string());
        assert_eq!(part_one(&data), 24000)
    }

    #[test]
    fn part_two_returns_correct_output() {
        let data = parse_input(INPUT.to_string());
        assert_eq!(part_two(&data), 45000)
    }
}