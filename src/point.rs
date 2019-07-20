use std::ops::{Sub, Add};
use crate::vector::Vector3;

#[derive(Clone, Debug)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3 {
    pub fn zero() -> Point3 {
        Point3::from_one(0.0)
    }
    pub fn from_one(v: f64) -> Point3 {
        Point3 { x: v, y: v, z: v }
    }
}

impl<'a, 'b> Sub<&'a Point3> for &'b Point3 {
    type Output = Vector3;

    fn sub(self, other: &'a Point3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a, 'b> Add<&'a Vector3> for &'b Point3 {
    type Output = Point3;

    fn add(self, other: &'a Vector3) -> Point3 {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}