use super::vector::Vector;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Direction {
    Up = 85,
    Down = 68,
    Left = 76,
    Right = 82,
}

impl std::convert::From<u8> for Direction {
    fn from(v: u8) -> Self {
        // hit it with a hammer
        unsafe { std::mem::transmute(v) }
    }
}

impl Direction {
    pub fn to_vector(&self) -> Vector {
        match self {
            Direction::Up => Vector::new(0, -1),
            Direction::Down => Vector::new(0, 1),
            Direction::Left => Vector::new(-1, 0),
            Direction::Right => Vector::new(1, 0),
        }
    }
}
