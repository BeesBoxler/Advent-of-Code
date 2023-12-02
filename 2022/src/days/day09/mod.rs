mod direction;
mod vector;

use direction::Direction;
use std::collections::HashSet;
use vector::Vector;

type Path = HashSet<Vector>;
type Bounds = [Vector; 2];

pub fn run(input: String) {
    println!(
        "Tail of rope length two touches {} squares",
        part_one(&input)
    );
    println!(
        "Tail of rope length nine touches {} squares",
        part_two(&input)
    );
}

fn part_one(input: &String) -> u32 {
    let instructions = parse_input(input);

    calculate_rope(&instructions, 2)
}

fn part_two(input: &String) -> u32 {
    let instructions = parse_input(input);
    calculate_rope(&instructions, 10)
}

fn calculate_rope(instructions: &Vec<Direction>, rope_length: usize) -> u32 {
    let mut rope = vec![Vector::zero(); rope_length];
    let mut tail_path = HashSet::from([Vector::zero()]);
    let mut bounds = [Vector::zero(), Vector::zero()];

    // Just for drawing. Find the size of the board
    for instruction in instructions {
        let instruction_vector = instruction.to_vector();
        rope[0].add_in_place(&instruction_vector);
        update_bounds(&rope[0], &mut bounds);
    }

    rope[0] = Vector::zero();

    for instruction in instructions {
        let instruction_vector = instruction.to_vector();
        rope[0].add_in_place(&instruction_vector);

        for i in 1..rope.len() {
            if !rope[i].is_adjacent_to(&rope[i - 1]) {
                let tail_move = get_move(&rope[i], &rope[i - 1]);
                rope[i].add_in_place(&tail_move);
                update_bounds(&rope[i], &mut bounds);
            }
        }

        update_bounds(&rope[0], &mut bounds);
        tail_path.insert(rope[rope_length - 1].clone());

        if !cfg!(test) && bounds[1].x - bounds[0].x < 160 {
            draw_path(&tail_path, &bounds, Some(&rope));
        }
    }

    if !cfg!(test) && bounds[1].x - bounds[0].x < 160 {
        draw_path(&tail_path, &bounds, Some(&rope));
    }

    tail_path.len() as u32
}

fn get_move(tail: &Vector, head: &Vector) -> Vector {
    Vector {
        x: (tail.x - head.x).clamp(-1, 1) * -1,
        y: (tail.y - head.y).clamp(-1, 1) * -1,
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
fn draw_path(path: &Path, bounds: &Bounds, rope: Option<&Vec<Vector>>) {
    print!("{esc}[?25l{esc}[2J{esc}[1;1H", esc = 27 as char);
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

    if let Some(rope) = rope {
        for i in (0..rope.len()).rev() {
            let x = (rope[i].x + delta_x) as usize;
            let y = (rope[i].y + delta_y) as usize;
            let symbol = if i == 0 {
                'H'
            } else {
                char::from((i + 48) as u8)
            };
            field[y][x] = symbol;
        }
    }

    field[delta_y as usize][delta_x as usize] = 's';

    for row in field {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
    let mut child = std::process::Command::new("sleep")
        .arg("0.1")
        .spawn()
        .unwrap();
    let _result = child.wait().unwrap();
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

    const INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(&INPUT.to_string()), 13);
        assert_eq!(part_one(&INPUT_2.to_string()), 88);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(&INPUT.to_string()), 1);
        assert_eq!(part_two(&INPUT_2.to_string()), 36);
    }
}
