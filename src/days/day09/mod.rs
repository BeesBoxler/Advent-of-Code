mod direction;
mod vector;

use vector::Vector;
use direction::Direction;
use std::collections::HashSet;

type Path = HashSet<Vector>;
type Bounds = [Vector; 2];

pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &String) -> u32 {
    let instructions = parse_input(input);
    let mut head = Vector::new(0, 0);
    let mut tail = head.clone();
    let mut tail_path = HashSet::from([tail.clone()]);
    let mut bounds = [Vector::zero(), Vector::zero()];

    // Just for drawing. Find the size of the board
    for instruction in &instructions {
        let instruction_vector = instruction.to_vector();
        head.add_in_place(&instruction_vector);
        update_bounds(&head, &mut bounds);
    }

    head = Vector::zero();

    for instruction in &instructions {
        let instruction_vector = instruction.to_vector();
        head.add_in_place(&instruction_vector);

        if !tail.is_adjacent_to(&head) {
            let tail_move = get_move(&tail, &head);
            tail.add_in_place(&tail_move);
            tail_path.insert(tail.clone());
            update_bounds(&tail, &mut bounds);
        }

        update_bounds(&head, &mut bounds);
        // draw_path(&tail_path, &bounds, Some(&tail), Some(&head));
    }

    // draw_path(&tail_path, &bounds, None, None);

    tail_path.len() as u32
}

fn get_move(tail: &Vector, head: &Vector) -> Vector {
    Vector {
        x: (tail.x - head.x).clamp(-1, 1)*-1,
        y: (tail.y - head.y).clamp(-1,1)*-1

    }
}

fn update_bounds(point: &Vector, bounds: &mut Bounds) {
    if point.x < bounds[0].x {
        bounds[0].x = point.x;
    } else if point.x > bounds[1].x {
        bounds[1].x = point.x;
    }

    if point.y < bounds[0].y {
        bounds[0].y = point.y;
    } else if point.y > bounds[1].y {
        bounds[1].y = point.y;
    }
}

#[allow(dead_code)]
fn draw_path(path: &Path, bounds: &Bounds, tail: Option<&Vector>, head: Option<&Vector>) {
    let delta_x = bounds[0].x.abs();
    let delta_y = bounds[0].y.abs();
    let width: usize = (delta_x + bounds[1].x + 1) as usize;
    let height: usize = (delta_y + bounds[1].y + 1) as usize;

    let mut field = vec![vec!['.'; width]; height];

    for point in path {
        let x = (point.x + delta_x) as usize;
        let y = (point.y + delta_y) as usize;
        field[y][x] = '#';
    }

    if let Some(tail) = tail {
        let x = (tail.x + delta_x) as usize;
        let y = (tail.y + delta_y) as usize;
        field[y][x] = 'T';
    }

    if let Some(head) = head {
        let x = (head.x + delta_x) as usize;
        let y = (head.y + delta_y) as usize;
        field[y][x] = 'H';
    }

    let length = field.len() - 1;
    field[length][0] = 's';

    for row in field {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn part_two(input: &String) -> u32 {
    0
}

fn parse_input(input: &String) -> Vec<Direction> {
    input
        .lines()
        .map(|line| line.split_once(' '))
        .fold(Vec::new(), |mut directions, line| {
            let line = line.unwrap();
            for _ in 0..line.1.parse::<u8>().unwrap() {
                // Dangerous call. If line.0 isn't one of the four characters will eit with SIGTRAP
                directions.push(line.0.as_bytes()[0].into());
            }

            directions
        })
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(&INPUT.to_string()), 13);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(&INPUT.to_string()), 0);
    }
}
