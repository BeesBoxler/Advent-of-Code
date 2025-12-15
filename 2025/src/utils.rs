pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn from(x: &usize, y: &usize) -> Self {
        Self {
            x: x.clone(),
            y: y.clone(),
        }
    }
}
