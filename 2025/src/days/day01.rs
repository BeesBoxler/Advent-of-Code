const MAX: usize = 100;

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: usize,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let (direction, value) = value.split_at(1);

        Instruction {
            direction: direction.into(),
            amount: value.parse().unwrap(),
        }
    }
}

impl Instruction {
    fn execute(&self, current: usize) -> usize {
        self.direction.execute(self.amount, current)
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("something has gone terribly wrong"),
        }
    }
}

impl Direction {
    fn execute(&self, a: usize, b: usize) -> usize {
        match self {
            Direction::Left => {
                if b == 0 {
                    MAX - a % MAX
                } else if a > b {
                    MAX - (a % b)
                } else {
                    b - a
                }
            }
            Direction::Right => (a + b) % MAX,
        }
    }
}

pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> u32 {
    let mut count = 0;
    let mut current = 50;
    let instructions: Vec<Instruction> = input.lines().map(|line| line.into()).collect();

    for instruction in instructions {
        current = instruction.execute(current);

        if current == 0 {
            count += 1;
        }
    }

    count
}

fn part_two(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(INPUT), 3);
    }

    #[test]
    #[ignore]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 0);
    }
}
