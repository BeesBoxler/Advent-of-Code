use crate::utils::Point;

pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

type Map = Vec<Vec<char>>;

fn parse_input(input: &str) -> Map {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn count_neighbours(map: &Map, position: &Point) -> usize {
    let mut count = 0;
    if position.x > 0 {
        if map[position.y][position.x - 1] == '@' {
            count += 1
        }
        if position.y > 0 && map[position.y - 1][position.x - 1] == '@' {
            count += 1
        }
        if position.y < map.len() - 1 && map[position.y + 1][position.x - 1] == '@' {
            count += 1
        }
    }
    if position.x < map[0].len() - 1 {
        if map[position.y][position.x + 1] == '@' {
            count += 1
        }
        if position.y > 0 && map[position.y - 1][position.x + 1] == '@' {
            count += 1
        }
        if position.y < map.len() - 1 && map[position.y + 1][position.x + 1] == '@' {
            count += 1
        }
    }
    if position.y > 0 && map[position.y - 1][position.x] == '@' {
        count += 1
    }
    if position.y < map.len() - 1 && map[position.y + 1][position.x] == '@' {
        count += 1
    }
    count
}

fn part_one(input: &str) -> usize {
    let map = parse_input(input);

    let mut count = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '@' && count_neighbours(&map, &Point { x, y }) < 4 {
                count += 1
            }
        }
    }

    count
}

fn part_two(input: &str) -> u32 {
    let mut map = parse_input(input);

    let mut count = 0;
    for _ in 0..1000 {
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == '@' && count_neighbours(&map, &Point { x, y }) < 4 {
                    count += 1;
                    map[y][x] = 'x'
                }
            }
        }
    }

    // draw_map(&map);

    count
}

#[allow(dead_code)]
fn draw_map(map: &Map) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", map[y][x]);
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(INPUT), 13);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 43);
    }
}

