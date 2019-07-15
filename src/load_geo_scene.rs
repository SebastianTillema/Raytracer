use crate::point::Point;
use crate::scene::{Element, Scene, Triangle};
use std::fs::File;
use std::io::prelude::*;

pub struct GeoData {
    num_face: usize,
    face_index_array: Vec<usize>,
    vertex_index_array: Vec<usize>,
    vertex_array: Vec<Point>,
    normal_array: Vec<Point>,
}

pub fn load_geo_file(file_path: String) -> std::io::Result<GeoData> {
    let mut file = File::open(file_path)?;
    let mut file_as_string = String::new();
    file.read_to_string(&mut file_as_string)?;

    // spliting data
    let content: Vec<&str> = file_as_string.split('\n').collect();

    // number of faces
    let num_face: usize = content[0].parse::<usize>().unwrap();

    // face index array
    let string_index_array = content[1].parse::<String>().unwrap();
    let face_index_array: Vec<usize> = string_index_array
        .split(' ') // split string of numbers
        .map(|s| s.parse().unwrap()) // pares strings to numbers
        .collect(); // collect to a vector;

    // vertex index array
    let string_vertex_index = content[2].parse::<String>().unwrap();
    let vertex_index_array: Vec<usize> = string_vertex_index
        .split(' ') // split string of numbers
        .map(|s| s.parse().unwrap()) // pares strings to numbers
        .collect(); // collect to a vector;

    // vertices array (3 coordinates for each vertex index)
    let string_coordinate = content[3].parse::<String>().unwrap();
    let coordinate_array: Vec<f64> = string_coordinate
        .split(' ') // split string of numbers
        .map(|s| s.parse().unwrap()) // pares strings to numbers
        .collect(); // collect to a vector;

    let mut vertex_array: Vec<Point> = Vec::new();
    for i in 0..(coordinate_array.len() / 3) {
        vertex_array.push(Point {
            x: coordinate_array[3 * i],
            y: coordinate_array[3 * i + 1] - 8.0,  // todo
            z: coordinate_array[3 * i + 2] - 12.0, // todo: -10 test
        });
    }

    // normal array
    let string_normals = content[4].parse::<String>().unwrap();
    let coordinate_array2: Vec<f64> = string_normals
        .split(' ') // split string of numbers
        .map(|s| s.parse().unwrap()) // pares strings to numbers
        .collect(); // collect to a vector;
    let mut normal_array: Vec<Point> = Vec::new();

    for i in 0..(coordinate_array2.len() / 3) {
        normal_array.push(Point {
            x: coordinate_array2[3 * i],
            y: coordinate_array2[3 * i + 1],
            z: coordinate_array2[3 * i + 2],
        });
    }
    let res: GeoData = GeoData {
        num_face: num_face,
        face_index_array: face_index_array,
        vertex_index_array: vertex_index_array,
        vertex_array: vertex_array,
        normal_array: normal_array,
    };
    Ok(res)
}

pub fn create_trianglemesh(geo_data: &GeoData) -> Vec<usize> {
    let mut triangle_index_array: Vec<usize> = Vec::new();
    let mut k = 0;
    for i in 0..geo_data.num_face {
        // for each face
        for j in 0..geo_data.face_index_array[i] - 2 {
            // for each triangle in the face
            triangle_index_array.push(geo_data.vertex_index_array[k]);
            triangle_index_array.push(geo_data.vertex_index_array[k + j + 1]);
            triangle_index_array.push(geo_data.vertex_index_array[k + j + 2]);
        }
        k += geo_data.face_index_array[i]; // continue to next face's vertices
    }
    triangle_index_array
}

pub fn create_triangles(
    vertex_array: Vec<Point>,
    triangle_index_array: Vec<usize>,
) -> Vec<Element> {
    let mut triangles: Vec<Element> = Vec::new();
    for i in 0..triangle_index_array.len() / 3 {
        let triangle: Triangle = Triangle {
            point1: vertex_array
                .get(*triangle_index_array.get(3 * i).unwrap())
                .unwrap()
                .clone(),
            point2: vertex_array
                .get(*triangle_index_array.get(3 * i + 1).unwrap())
                .unwrap()
                .clone(),
            point3: vertex_array
                .get(*triangle_index_array.get(3 * i + 2).unwrap())
                .unwrap()
                .clone(),
        };
        triangles.push(Element::Triangle(triangle));
    }
    triangles
}

/** Blob */
pub fn create_scene_from_file() -> std::io::Result<Scene> {
    // load file
    let file_content = load_geo_file(String::from("geo_test.geo"));
    let geo_data: GeoData;
    match file_content {
        Ok(data) => geo_data = data,
        Err(e) => {
            println!("error parsing file: {:?}", e);
            return Err(e);
        }
    }
    // get triangle data
    let triangle_index_array: Vec<usize> = create_trianglemesh(&geo_data);
    let triangles: Vec<Element> = create_triangles(geo_data.vertex_array, triangle_index_array);
    let res: Scene = Scene {
        width: 600,
        height: 600,
        fov: 90.0,
        elements: triangles,
    };
    Ok(res)
}

#[cfg(test)]
mod test_file_read {
    use super::*;

    #[test]
    fn negativ_load_geo_file() {
        let file_content = load_geo_file(String::from("file_that_does_not_exist.geo"));
        match file_content {
            Ok(c) => assert!(false),
            Err(e) => assert!(true),
        }
    }

    #[test]
    fn positiv_create_triangles() {
        let point1: Point = Point {
            x: -1.0,
            y: -1.0,
            z: -5.0,
        };
        let point2: Point = Point {
            x: -1.0,
            y: 1.0,
            z: -5.0,
        };
        let point3: Point = Point {
            x: 1.0,
            y: 1.0,
            z: -5.0,
        };
        let point4: Point = Point {
            x: 1.0,
            y: -1.0,
            z: -5.0,
        };

        let vertex_array: Vec<Point> = vec![
            point1.clone(),
            point2.clone(),
            point3.clone(),
            point4.clone(),
        ];
        let triangle_index_array: Vec<usize> = vec![0, 1, 2, 0, 2, 3];
        let actual: Vec<Element> = create_triangles(vertex_array, triangle_index_array);

        let expected: Vec<Triangle> = vec![
            Triangle {
                point1: point1.clone(),
                point2: point2.clone(),
                point3: point3.clone(),
            },
            Triangle {
                point1: point1.clone(),
                point2: point3.clone(),
                point3: point4.clone(),
            },
        ];

        for i in 0..actual.len() {
            match &&actual[i] {
                &Element::Triangle(t) => {
                    assert!(t.point1.x == expected[i].point1.x);
                    assert!(t.point1.y == expected[i].point1.y);
                    assert!(t.point1.z == expected[i].point1.z);
                    assert!(t.point2.x == expected[i].point2.x);
                    assert!(t.point2.y == expected[i].point2.y);
                    assert!(t.point2.z == expected[i].point2.z);
                    assert!(t.point3.x == expected[i].point3.x);
                    assert!(t.point3.y == expected[i].point3.y);
                    assert!(t.point3.z == expected[i].point3.z);
                }
                _ => assert!(false),
            }
        }
    }
}
