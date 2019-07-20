use crate::point::Point3; // get access to point struct
use crate::vector::Vector3;
use crate::intersection::{Ray, Intersectable};

pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub color: Color,
}

pub struct Triangle {
    pub point1: Point3,
    pub point2: Point3,
    pub point3: Point3,
    pub color: Color,
}

pub struct Plane {
    pub origin: Point3, 
    pub normal: Vector3,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>, 
}

pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
    Triangle(Triangle),
}

impl Triangle {
    pub fn calculate_normal(&self) -> Vector3 {
        let vec1 = self.point2.to_vector() - self.point1.to_vector();
        let vec2 = self.point3.to_vector() - self.point1.to_vector();
        vec1.cross(&vec2)
    }
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<&Element> {
        let mut nearest_element: Option<&Element> = None;
        let mut dist_to_nearest_element: f64 = 10E6;
        for e in &self.elements {
            let intersect = e.intersect(&ray);
            // find nearest element
            match intersect {
                Some(d) => {
                    if d < dist_to_nearest_element {
                        nearest_element = Some(e);
                        dist_to_nearest_element = d;
                    }
                }
                None => {}
            }
        }
        nearest_element
    }
}

