use std::ops::{Sub, Add};
use crate::vector::Vector3;

#[derive(Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn zero() -> Point {
        Point::from_one(0.0)
    }
    pub fn from_one(v: f64) -> Point {
        Point { x: v, y: v, z: v }
    }
}

impl<'a, 'b> Sub<&'a Point> for &'b Point {
    type Output = Vector3;

    fn sub(self, other: &'a Point) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a, 'b> Add<&'a Vector3> for &'b Point {
    type Output = Point;

    fn add(self, other: &'a Vector3) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}