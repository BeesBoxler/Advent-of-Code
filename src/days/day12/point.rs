use core::fmt;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn into(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    x: i8,
    y: i8,
}

impl Point {
    pub fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }

    pub fn from_usize(x: usize, y: usize) -> Self {
        Self::new(x as i8, y as i8)
    }

    pub fn from_direction(direction: &Direction) -> Self {
        match direction {
            Direction::Up => Self::up(),
            Direction::Down => Self::down(),
            Direction::Left => Self::left(),
            Direction::Right => Self::right(),
        }
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    pub fn up() -> Self {
        Self::new(0, -1)
    }

    pub fn down() -> Self {
        Self::new(0, 1)
    }

    pub fn left() -> Self {
        Self::new(-1, 0)
    }

    pub fn right() -> Self {
        Self::new(1, 0)
    }

    pub fn get_x(&self) -> i8 {
        self.x
    }

    pub fn get_y(&self) -> i8 {
        self.y
    }

    pub fn add(&self, other: &Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn add_in_place(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl core::fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
