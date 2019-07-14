use crate::point::Point; // get access to point struct
use crate::vector::Vector3;

pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

pub struct Triangle {
    pub point1: Point,
    pub point2: Point,
    pub point3: Point,
}

pub struct Plane {
    pub origin: Point, 
    pub normal: Vector3,
}

pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
    Triangle(Triangle),
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>, 
}



