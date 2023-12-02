mod point;

use crate::utils::clear_screen;
use point::{Direction, Point};

const START: u8 = 65;
const END: u8 = 69; //nice

type Map = Vec<Vec<char>>;

pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &String) -> u32 {
    let map = parse_input(input);
    let (start, end) = dbg!(get_start_and_end(&map));
    let mut path = Vec::new();

    find_path(&map, &mut path, &start, &end);

    0
}

fn part_two(input: &String) -> u32 {
    0
}

fn find_path(map: &Map, path: &mut Vec<Direction>, start: &Point, end: &Point) {
    draw_path(start, end, path, map);
}

fn draw_path(start: &Point, end: &Point, path: &Vec<Direction>, map: &Map) {
    let mut new_map = vec![vec!['.'; map[0].len()]; map.len()];
    let mut curr = start.clone();

    for direction in path {
        curr.add_in_place(&Point::from_direction(direction));
        new_map[curr.get_y() as usize][curr.get_x() as usize] = direction.into();
    }
}

fn draw_map(map: &Map) {
    for row in map {
        for point in row {
            print!("{}", *point);
        }
        println!();
    }
}

fn get_start_and_end(map: &Map) -> (Point, Point) {
    let mut start = Point::zero();
    let mut end = Point::zero();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == START as char {
                start = Point::from_usize(x, y);
            } else if map[y][x] == END as char {
                end = Point::from_usize(x, y);
            }
        }
    }
    (start, end)
}

fn parse_input(input: &String) -> Map {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    #[ignore]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(&INPUT.to_string()), 31);
    }

    #[test]
    #[ignore]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(&INPUT.to_string()), 0);
    }
}
