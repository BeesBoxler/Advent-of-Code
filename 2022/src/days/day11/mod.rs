use self::monkey::Monkey;

mod monkey;

pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &String) -> usize {
    let mut monkeys = parse_input(input);
    let inspection_count = get_inspection_count(&mut monkeys, 20, 3);

    &inspection_count[0] * &inspection_count[1]
}

fn part_two(input: &String) -> usize {
    let mut monkeys = parse_input(input);
    let inspection_count = get_inspection_count(&mut monkeys, 10000, 1);

    inspection_count[0] * inspection_count[1]
}

fn get_inspection_count(monkeys: &mut Vec<Monkey>, count: usize, div: u32) -> Vec<usize> {
    let gcd = monkeys
        .iter()
        .fold(1, |acc, monkey| acc * monkey.get_divisor());
    let mut inspection_count = vec![0; monkeys.len()];
    for _ in 0..count {
        for i in 0..monkeys.len() {
            for item in monkeys[i].get_items() {
                inspection_count[i] += 1;
                let target = monkeys[i].calculate_recipient(item, div, gcd) as usize;
                let new_worry_level = monkeys[i].calculate_new_worry_level(item, div, gcd);
                monkeys[target].add_item(new_worry_level);
                monkeys[i].remove_item(item);
            }
        }
    }

    inspection_count.sort_by(|a, b| b.cmp(a));
    inspection_count
}

fn parse_input(input: &String) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    for definition in input.split("\n\n") {
        monkeys.push(Monkey::from_definition(definition));
    }

    monkeys
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(&INPUT.to_string()), 10605);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(&INPUT.to_string()), 2713310158);
    }
}
