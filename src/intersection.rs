use crate::point::Point;
use crate::scene::{Color, Element, Plane, Scene, Sphere, Triangle};
use crate::vector::Vector3;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        //assert!(scene.width >= scene.height);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x =
            ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

        Ray {
            origin: Point::zero(),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }
            .normalize(),
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}

impl Intersectable for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref s) => s.intersect(ray),
            Element::Triangle(ref s) => s.intersect(ray),
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        // create a line segment between the ray origin and the center of the sphere
        let l1_vector: Vector3 = &self.center - &ray.origin;

        // use l1 as a hypotenuse and find the length of the adjacent side
        let l1_length: f64 = l1_vector.dot(&ray.direction);

        // find the length-squared of the opposite side. (pythagoras, l1_length^2 - l2_length^2 = l3_length^2)
        // this is equivalent to (but faster than) (l.length() * l.length()) - (adj2 * adj2)
        let l2_length = l1_vector.dot(&l1_vector) - (l1_length * l1_length);

        let radius2 = self.radius * self.radius;

        //If that length is greater than radius, the ray does not intersects the sphere
        if l2_length > self.radius {
            return None;
        }
        let thc = (radius2 - l2_length).sqrt();
        let t0 = l1_length - thc;
        let t1 = l1_length + thc;

        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }
        // distance from sphere to camara (used to find the right element to render in front)
        let distance = if t0 < t1 { t0 } else { t1 };

        Some(distance)
    }
}

impl Intersectable for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        // self, a triangle with 3 points A, B and C
        let AB: Vector3 = &self.point2 - &self.point1;
        let AC: Vector3 = &self.point3 - &self.point1;

        // normal vector of triangle plan
        let normal: Vector3 = AB.cross(&AC).normalize();

        // check if ray and triangle plane is parallel
        let normal_dot_raydirection = normal.dot(&ray.direction);
        if normal_dot_raydirection.abs() < 1e-6 {
            // close to 0
            // print!("Parallel", );
            return None;
        }

        // distance to the intersection from the ray's origin to the triangle plane
        let dist: f64 =
            normal.x * self.point1.x + normal.y * self.point1.y + normal.z * self.point1.z;
        let t: f64 = (dist
            - (normal.x * ray.origin.x + normal.y * ray.origin.y + normal.z * ray.origin.z))
            / normal_dot_raydirection;

        // check if triangle is behind the ray
        if t < 0.0 {
            // print!("Behind: {} \n", t);
            return None;
        }

        // compute intersection point
        let p: Point = &ray.origin + &(&ray.direction * t); //TODO:

        // inside-outside test
        let mut c: Vector3;

        // edge 0
        let edge0: Vector3 = &self.point2 - &self.point1;
        let vp0 = &p - &self.point1;
        c = edge0.cross(&vp0);
        if c.dot(&normal) < 0.0 {
            return None; //Some(255.0);
        }

        // edge 1
        let edge1: Vector3 = &self.point3 - &self.point2;
        let vp1 = &p - &self.point2;
        c = edge1.cross(&vp1);
        if c.dot(&normal) < 0.0 {
            return None; //Some(255.0);
        }

        // edge 2
        let edge2: Vector3 = &self.point1 - &self.point3;
        let vp2 = &p - &self.point3;
        c = edge2.cross(&vp2);
        if c.dot(&normal) < 0.0 {
            return None; //Some(255.0);
        }
        Some(10.0)
    }
}
// math: https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection
impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction); // l·n
        if denom > 1e-6 {
            // not 0 because of floating-point error
            // calculate interscetion point
            let v: Vector3 = &self.origin - &ray.origin; // (p0-l0)
            let d: f64 = v.dot(&normal) / denom; // ((p0-l0)·n) / l·n
            let intersection = &ray.origin + &(&self.normal * d);
            if d > 0.0 {
                return Some((intersection.z * 100.0).abs());
            }
        }
        None // ray and plane are parallel
    }
}

pub fn get_color(element: &Element) -> &Color {
    match element {
        Element::Triangle(t) => {
            return &t.color;
        },
        _ => {print!("Not Triangle \n", )},
    }
    &Color {
        red: 65.0,
        green: 20.0,
        blue: 150.0,
    }
}

#[cfg(test)]
mod test_rendering {
    use super::*;
    use crate::scene::*;

    #[test]
    fn intersect_sphere_ray() {
        let sphere: Sphere = Sphere {
            center: Point {
                x: 1.0,
                y: 1.0,
                z: -5.0,
            },
            radius: 5.0,
            color: Color {
                red: 155.0,
                green: 155.0,
                blue: 255.0,
            },
        };
        // ray hits center of sphere
        let prime_ray: Ray = Ray {
            origin: Point {
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
        let intersection: bool = match sphere.intersect(&prime_ray) {
            Some(x) => true,
            None => false,
        };
        assert!(intersection);
    }
}
