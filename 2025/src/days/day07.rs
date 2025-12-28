use std::fmt::Display;

pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}
#[derive(PartialEq, Eq, Copy, Clone)]
enum Tile {
    Empty,
    Origin,
    Beam,
    Splitter,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            'S' => Self::Origin,
            '|' => Self::Beam,
            '^' => Self::Splitter,
            x => panic! {"Invalid tile: {x}"},
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => ' ',
                Tile::Origin => 'S',
                Tile::Beam => '|',
                Tile::Splitter => '^',
            }
        )
    }
}
struct Map(Vec<Vec<Tile>>);

impl Map {
    fn iterate(&mut self) -> usize {
        let mut count = 0;
        for y in 1..self.0.len() {
            for x in 0..self.0[0].len() {
                if self.0[y][x] == Tile::Empty {
                    if self.0[y - 1][x] == Tile::Origin || self.0[y - 1][x] == Tile::Beam {
                        self.0[y][x] = Tile::Beam;
                    }
                } else if self.0[y][x] == Tile::Splitter && self.0[y - 1][x] == Tile::Beam {
                    count += 1;
                    self.0[y][x - 1] = Tile::Beam;
                    self.0[y][x + 1] = Tile::Beam;
                }
            }
        }

        count
    }

    fn find_routes(&mut self) -> usize {
        let mut count = vec![0; self.0[0].len()];

        for y in 1..self.0[0].len() {
            for x in 0..self.0[y].len() {
                let above = self.0[y - 1][x];
                let pos = self.0[y][x];

                if above == Tile::Origin && pos == Tile::Beam {
                    count[x] += 1;
                } else if above == Tile::Beam && pos == Tile::Splitter {
                    count[x - 1] += count[x];
                    count[x + 1] += count[x];
                    count[x] = 0;
                }
            }
        }

        count.iter().sum()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            for v in line {
                write!(f, "{v}")?;
            }
            writeln!(f)?;
        }

        writeln!(f)
    }
}

fn parse_input(input: &str) -> Map {
    Map(input
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect())
}

fn part_one(input: &str) -> usize {
    let mut map = parse_input(input);

    map.iterate()
}

fn part_two(input: &str) -> usize {
    let mut map = parse_input(input);
    map.iterate();
    map.find_routes()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(INPUT), 21);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 40);
    }
}
