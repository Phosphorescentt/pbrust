pub mod math;
pub mod objects;
pub mod operations;

use objects::{Camera, Colour, Material, Scene, Sphere, SphereLight, Vector3, Vector4};
use std::f32::consts::PI;

fn main() {
    let sphere = Sphere {
        position: Vector3(5.0, 0.0, 0.0),
        radius: 1.0,
        material: Material {
            colour: Colour(255, 255, 255),
        },
    };

    // let sun = SphereLight {
    //     shape: Sphere {
    //         position: Vector3(3.0, 2.0, 2.0),
    //         radius: 1.0,
    //     },
    //     material: Material {
    //         colour: Vector4(1.0, 1.0, 1.0, 1.0),
    //     },
    // };

    let scene = Scene {
        objects: vec![sphere],
    };

    let dt = chrono::offset::Utc::now();
    let camera = Camera {
        position: Vector3(0.0, 0.0, 0.0),
        view_direction: Vector3(1.0, 0.0, 0.0),
        horizontal_fov: PI / 4.0,
        vertical_fov: PI / 4.0,
        resolution: (1920, 1080),
        scene: scene,
        filename: format!("outputs/out@{}.png", dt.to_rfc3339()).to_owned(),
    };

    camera.render();
}
