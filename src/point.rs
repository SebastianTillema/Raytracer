use crate::vector::{Vector3, Matrix3};
use std::ops::{Add, Sub, Mul};

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

    pub fn to_vector(&self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
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

impl Mul<&Matrix3> for Point3 {
    type Output = Point3;

    fn mul(self, other: &Matrix3) -> Point3 {
        Point3 {
            x: other.vec1.x * self.x + other.vec1.y * self.y + other.vec1.z * self.z, 
            y: other.vec2.x * self.x + other.vec2.y * self.y + other.vec2.z * self.z, 
            z: other.vec3.x * self.x + other.vec3.y * self.y + other.vec3.z * self.z, 
        }
    }
}

#[cfg(test)]
mod test_point {
    use super::*;
    #[test]
    fn point_mul_matrix() {
        let point: Point3 = Point3 {
            x: 7.0,
            y: 17.0,
            z: 67.0,
        };
        let vec1: Vector3 = Vector3 {
            x: 7.0,
            y: 17.0,
            z: 67.0,
        };
        let vec2: Vector3 = Vector3 {
            x: 7.0,
            y: 17.0,
            z: 67.0,
        };
        let vec3: Vector3 = Vector3 {
            x: 7.0,
            y: 17.0,
            z: 67.0,
        };
        let matrix: Matrix3 = Matrix3 {
            vec1: vec1,
            vec2: vec2,
            vec3: vec3,
        };
        let res: Point3 = point * &matrix;
        print!("{:?}", res);
    }
    
}
