#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

use std::ops;

impl ops::Add<Self> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl ops::AddAssign<Self> for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
