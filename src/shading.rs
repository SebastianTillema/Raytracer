use crate::intersection::Ray;
use crate::point::Point3;
use crate::vector::Vector3;
use std::f64;

pub fn facing_ratio(ray: &Ray, normal: &Vector3) -> f64 {
    let ratio: f64 = ((normal.normalize()).dot(&ray.direction.normalize())).abs();
    let res = ratio.max(0.0);
    res
}

#[cfg(test)]
mod test_shading {
    use super::*;
    
    #[test]
    fn dummy() {
        let ray = Ray {
            origin: Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vector3 {
                x: 1.0,
                y: 1.0,
                z: -5.0,
            },            
        };
        let normal = Vector3 {
            x: 0.8,
            y: 0.8,
            z: -5.0,
        };
        let x = facing_ratio(&ray, &normal);
    }
}