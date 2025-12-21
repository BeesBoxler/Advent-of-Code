pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn from(v: &str) -> Self {
        match v {
            "*" => Operator::Multiply,
            "+" => Operator::Add,
            x => panic!("Invalid operator {x}"),
        }
    }
}

#[derive(Debug)]
struct Problem {
    operands: Vec<usize>,
    operator: Operator,
}

impl Problem {
    fn execute(self) -> usize {
        self.operands
            .into_iter()
            .reduce(|total, n| match self.operator {
                Operator::Add => total + n,
                Operator::Multiply => total * n,
            })
            .unwrap_or(0)
    }
}

fn parse_input(input: &str) -> Vec<Problem> {
    let array: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let mut problems = vec![];

    for i in 0..array[0].len() {
        let mut operands = vec![];
        'row: for j in 0..array.len() {
            if let Ok(operand) = array[j][i].parse::<usize>() {
                operands.push(operand);
            } else {
                problems.push(Problem {
                    operands,
                    operator: Operator::from(array[j][i]),
                });
                break 'row;
            }
        }
    }

    problems
}

fn part_one(input: &str) -> usize {
    parse_input(input)
        .into_iter()
        .map(|problem| problem.execute())
        .sum()
}

fn part_two(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(INPUT), 4277556);
    }

    #[test]
    #[ignore]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 0);
    }
}

