pub mod rendering;
pub mod scene;
pub mod point;
pub mod vector;
pub mod load_geo_scene;

use image::{DynamicImage, GenericImageView, GenericImage, Rgba, ImageBuffer};
use scene::{Scene, Color, Sphere, Element, Plane, Triangle};
use point::Point;
use rendering::{Ray, Intersectable, get_color};
use vector::Vector3;
use load_geo_scene::create_scene_from_file;

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_bgr8(scene.width, scene.height);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);
            let mut nearest_element: Option<&Element> = None; 
            let mut dist_to_nearest_element: f64 = 10E6;
            for e in &scene.elements {
                let intersect = e.intersect(&ray);
                // render only the nearest element
                match intersect {
                    Some(d) =>
                        if d < dist_to_nearest_element {
                            nearest_element = Some(e);
                            dist_to_nearest_element = d; 
                        },
                    None    => {},
                }

            }
            match nearest_element {
                    Some(c) => {
                        let color: Color = get_color(c);
                        // print!("dist: {} \n ", dist_to_nearest_element);
                        image.put_pixel(x, y, Rgba([(dist_to_nearest_element * 100.0) as u8, color.green as u8, color.blue as u8, 255]))
                    },
                    None    => {},
            }
        }
        print!("pixel x: {}\n", x);
    }
    image
}

pub fn save_image(image: &DynamicImage) {
    image.save("awesome.png").unwrap();
}

#[cfg(test)]
    mod integration_test {

    use super::*;
    #[test]
    fn test_can_render_triangle_scene() {
        let scene = Scene {
            width: 320,
            height: 240,
            fov: 90.0,
            elements: vec![Element::Triangle (
                Triangle {
                    point1: Point {
                        x: 1.0,
                        y: 0.0,
                        z: -5.0,
                    },
                    point2: Point {
                        x: 4.0,
                        y: 0.0,
                        z: -5.0,
                    },
                    point3: Point {
                        x: 0.0,
                        y: 1.0,
                        z: -5.0,
                    },
                }
            ),
            Element::Triangle (
                Triangle {
                    point1: Point {
                        x: -4.0,
                        y: 0.0,
                        z: -5.0,
                    },
                    point2: Point {
                        x: -1.0,
                        y: 0.0,
                        z: -5.0,
                    },
                    point3: Point {
                        x: -1.0,
                        y: 1.0,
                        z: -5.0,
                    },
                }
            ),
            ],
        };
        let image = render(&scene);  
        save_image(&image)
    }

    #[test]
    fn poly() {
        let scene = create_scene_from_file();
        let image: DynamicImage;
        match scene {
            Ok(s) => image = render(&s),
            Err(e) => return,
        }
        save_image(&image);
    }

    #[test]
    fn test_can_render_sphere_scene() {
        let scene = Scene {
            width: 800,
            height: 600,
            fov: 90.0,
            elements: vec![
                Element::Sphere ( 
                    Sphere {
                        center: Point {
                            x: -1.0,
                            y: 1.0,
                            z: -6.0,
                        },
                        radius: 5.0, 
                        color: Color {
                            red: 0.0,
                            green: 155.0,
                            blue: 0.0,
                        },
                    }, 
                ), 
                Element::Sphere ( 
                    Sphere {
                        center: Point {
                            x: 1.0,
                            y: 1.0,
                            z: -5.0,
                        },
                        radius: 5.0, 
                        color: Color {
                            red: 155.0,
                            green: 0.0,
                            blue: 0.0,
                        },
                    },     
                ),
            ],
        };
        let image = render(&scene);  
        save_image(&image)
    }

    #[test]
    fn test_can_render_plane_scene() {
        let scene = Scene {
            width: 800,
            height: 600,
            fov: 90.0,
            elements: vec![Element::Plane (
                Plane {
                    origin: Point {
                        x: 0.0,
                        y: 0.0,
                        z: -5.0,
                    },
                    normal: Vector3 {
                        x: -0.1,
                        y: -0.9,
                        z: -0.1,
                    },
                }
            ),
            ],
        };
        let image = render(&scene);  
        save_image(&image)
    }
}