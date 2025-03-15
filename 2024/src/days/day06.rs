pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

type Map = Vec<Vec<char>>;

fn part_one(input: &str) -> usize {
    let mut data: Map = input.lines().map(|line| line.chars().collect()).collect();
    let mut direction = Direction::North;
    let mut position = find_initial_position(&data).unwrap();

    while position.0 < data.len() && position.1 < data[0].len() && position.0 > 0 && position.1 > 0 {
        let next_position = direction.get_next_position(&position);
        if is_blocked(&data,&next_position) {
            direction = direction.turn();
        } else {
            data[position.0][position.1] = 'X';
            position = next_position;
        }
    }

    

    count_visited_squares(&data)
}

fn count_visited_squares(data: &Map) -> usize {
    let mut count = 0;
    for row in data {
        for item in row {
            if *item == 'X' {
                count += 1;
            }
        }
    }

    count
}

fn find_initial_position(data: &Map) -> Option<(usize, usize)> {
    for (i, row) in data.iter().enumerate() {
        for  (j, item) in row.iter().enumerate() {
            if *item == '^' {
                return Some((i,j));
            }
        }
    }

    None
}

fn is_blocked(data: &Map, position: &(usize, usize)) -> bool {
    if position.0 < data.len() && position.1 < data[0].len() {
        return data[position.0][position.1] == '#'
    }

    false
}

fn part_two(_input: &str) -> u32 {
    0
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn get_next_position(&self, position: &(usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (position.0-1, position.1),
            Direction::East => (position.0, position.1+1),
            Direction::South => (position.0+1, position.1),
            Direction::West => (position.0, position.1-1),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(&INPUT.to_string()), 41);
    }

    #[test]
    #[ignore]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(&INPUT.to_string()), 0);
    }
}