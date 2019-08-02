extern crate raytracer_lib;
use raytracer_lib::load_geo_scene;
use image::DynamicImage;

#[test]
    fn test_file_render() {
        let scene = load_geo_scene::create_scene_from_file(String::from("geometry/backdrop.geo"));
        let image: DynamicImage;
        match scene {
            Ok(s) => image = raytracer_lib::render(&s),
            Err(e) => return,
        }
        raytracer_lib::save_image(&image);
    }
    //  1m54.861s   shaders
    //  1m58.116s   no shader