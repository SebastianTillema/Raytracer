use crate::point::Point3;
use crate::vector::{Matrix3, Vector3};

pub enum Axis {
    x_axis,
    y_axis,
    z_axis,
}

pub fn rotate_object(points: Vec<Point3>, axis: Axis, degree: f64) -> Vec<Point3> {    
    let rotation_matrix; 
    match axis {
        Axis::x_axis => rotation_matrix = Matrix3 {
            vec1: Vector3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            vec2: Vector3 {
                x: 0.0,
                y: degree.cos(),
                z: -degree.sin(),
            },
            vec3: Vector3 {
                x: 0.0,
                y: degree.sin(),
                z: degree.cos(),
            },
        },
        Axis::y_axis => rotation_matrix = Matrix3 {
            vec1: Vector3 {
                x: degree.cos(),
                y: 0.0,
                z: degree.sin(),
            },
            vec2: Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            vec3: Vector3 {
                x: -degree.sin(),
                y: 0.0,
                z: degree.cos(),
            },
        },
        Axis::z_axis => rotation_matrix = Matrix3 {
            vec1: Vector3 {
                x: degree.cos(),
                y: -degree.sin(),
                z: 0.0,
            },
            vec2: Vector3 {
                x: degree.sin(),
                y: degree.cos(),
                z: 0.0,
            },
            vec3: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        },
    }

    let mut res: Vec<Point3> = Vec::new();
    
    for point in points {
        res.push(point * &rotation_matrix);
    }
   
    res
}
 

#[cfg(test)]
mod test_transforming {
    use super::*;
    #[test]
    fn matrix_rotation() {
        let point1: Point3 = Point3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let points: Vec<Point3> = vec!(point1);

        let res = rotate_object(points, Axis::z_axis, 1.5707);

        print!("{:?}", res)
        // assert!(res[0].x == 0.0);
        // assert!(res[0].y == 1.0);
        // assert!(res[0].x == 0.0);
    }
}