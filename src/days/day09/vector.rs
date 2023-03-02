#![allow(dead_code)]

type Number = i32;

#[derive(Hash, PartialEq, Eq)]
pub struct Vector {
    pub x: Number,
    pub y: Number,
}

impl Vector {
    pub fn new(x: Number, y: Number) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn zero() -> Self {
        Self::new(0,0)
    }

    pub fn add(&self, other: &Vector) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }

    pub fn scalar(&self, scale: Number) -> Vector {
        Self{
            x: self.x * scale,
            y: self.y * scale,
        }
    }

    pub fn is_adjacent_to(&self, other: &Self) -> bool {
        (self.x-1..=self.x+1).contains(&other.x)
            && (self.y-1..=self.y+1).contains(&other.y)
    }

    /*
     * Mutating Functions
     */

    pub fn add_in_place(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;

    }

    pub fn scalar_in_place(&mut self, scale: Number) {
        self.x *= scale;
        self.y *= scale;
    }

}

impl core::fmt::Debug for Vector{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl core::clone::Clone for Vector {
    fn clone(&self) -> Self {
        Self { x: self.x.clone(), y: self.y.clone() }
    }
}