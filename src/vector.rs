use std::ops::{Mul, Add, Sub};
use crate::point::Point3;

pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Matrix3 {
    pub vec1: Vector3,
    pub vec2: Vector3,
    pub vec3: Vector3,
}


impl Vector3 {
    pub fn zero() -> Vector3 {
        Vector3::from_one(0.0)
    }

    pub fn from_one(v: f64) -> Vector3 {
        Vector3 { x: v, y: v, z: v }
    }

    pub fn length(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normalize(&self) -> Vector3 {
        let inv_len = self.length().recip();
        Vector3 {
            x: self.x * inv_len,
            y: self.y * inv_len,
            z: self.z * inv_len,
        }
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn to_point(&self) -> Point3 {
        Point3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a, 'b> Mul<&'a Vector3> for &'b Vector3 {
    type Output = Vector3;

    fn mul(self, other: &'a Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<'a> Mul<f64> for &'a Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<'a> Mul<&'a Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, other: &'a Vector3) -> Vector3 {
        other * self
    }
}

impl Mul<&Matrix3> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: &Matrix3) -> Vector3 {
        Vector3 {
            x: other.vec1.x * self.x + other.vec1.y * self.y + other.vec1.z * self.z, 
            y: other.vec2.x * self.x + other.vec2.y * self.y + other.vec2.z * self.z, 
            z: other.vec3.x * self.x + other.vec3.y * self.y + other.vec3.z * self.z, 
        }
    }
}

#[cfg(test)]
mod test_vector {
    use super::*;

    #[test]
    fn matrix_mult() {
        
    }

    #[test]
    fn add_vector_vector() {
        let vec1: Vector3 = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let vec2: Vector3 = Vector3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let res: Vector3 = vec1 + vec2;
        assert_eq!(res.x, 5.0);
        assert_eq!(res.y, 7.0);
        assert_eq!(res.z, 9.0);
    }

    #[test]
    fn sub_vector_vector() {
        let vec1: Vector3 = Vector3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let vec2: Vector3 = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let res: Vector3 = vec1 - vec2;
        assert_eq!(res.x, 3.0);
        assert_eq!(res.y, 3.0);
        assert_eq!(res.z, 3.0);
    }

    #[test]
    fn mul_vector_vector() {
        let vec1: Vector3 = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let vec2: Vector3 = Vector3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let res: Vector3 = &vec1 * &vec2;
        assert_eq!(res.x, 4.0);
        assert_eq!(res.y, 10.0);
        assert_eq!(res.z, 18.0);
    }

    #[test]
    fn mul_vector_f64() {
        let vec1: Vector3 = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let sca: f64 = 2.5;
        let res: Vector3 = &vec1 * sca;
        assert_eq!(res.x, 2.5);
        assert_eq!(res.y, 5.0);
        assert_eq!(res.z, 7.5);
    }
    #[test]
    fn mul_f64_vector() {
        let vec1: Vector3 = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let sca: f64 = 2.5;
        let res: Vector3 = sca * &vec1;
        assert_eq!(res.x, 2.5);
        assert_eq!(res.y, 5.0);
        assert_eq!(res.z, 7.5);
    }
}