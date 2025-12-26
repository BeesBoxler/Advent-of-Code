pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

#[derive(Debug, Copy, Clone)]
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

    fn from_char(c: char) -> Self {
        match c {
            '*' => Operator::Multiply,
            '+' => Operator::Add,
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

fn parse_input_cephalopod_formatted(input: &str) -> Vec<Problem> {
    let mut problems = vec![];

    let lines = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let operator_index = lines.len() - 1;

    let mut values: Vec<usize> = vec![];
    let mut operator: Option<Operator> = None;
    for i in 0..lines[0].len() {
        if !lines[operator_index][i].is_whitespace() {
            operator = Some(Operator::from_char(lines[operator_index][i]));
        }

        let mut value = 0;

        if let Some(operator) = operator {
            for j in 0..operator_index {
                let b = lines[j][i] as usize;

                if b > 47 && b < 58 {
                    value *= 10;
                    value += b - 48;
                }
            }

            if value > 0 {
                values.push(value)
            }

            if value == 0 || i == lines[0].len() - 1 {
                problems.push(Problem {
                    operands: values,
                    operator: operator.clone(),
                });
                values = vec![];
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

fn part_two(input: &str) -> usize {
    parse_input_cephalopod_formatted(input)
        .into_iter()
        .map(|problem| problem.execute())
        .sum()
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
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 3263827);
    }
}

