pub mod math;
pub mod objects;
pub mod operations;

use math::Vector3;
use objects::{Camera, Colour, Material, Scene, Sphere};

use std::f32::consts::PI;

fn main() {
    let s1 = Sphere {
        position: Vector3(10.0, 2.0, 0.0),
        radius: 1.0,
        material: Material {
            colour: Colour(1.0, 0.1, 0.1),
            emitter: false,
        },
    };

    let s2 = Sphere {
        position: Vector3(10.0, 0.0, 0.0),
        radius: 1.0,
        material: Material {
            colour: Colour(0.1, 1.0, 0.1),
            emitter: false,
        },
    };

    let s3 = Sphere {
        position: Vector3(10.0, -2.0, 0.0),
        radius: 1.0,
        material: Material {
            colour: Colour(0.1, 0.1, 1.0),
            emitter: false,
        },
    };

    let sun = Sphere {
        position: Vector3(-5.0, 5.0, 0.0),
        radius: 5.0,
        material: Material {
            colour: Colour(1.0, 1.0, 1.0),
            emitter: true,
        },
    };

    let scene = Scene {
        objects: vec![s1, s2, s3, sun],
    };

    let dt = chrono::offset::Utc::now();
    let filename = format!("outputs/out@{}.png", dt.to_rfc3339());
    let camera = Camera {
        position: Vector3(0.0, 0.0, 0.0),
        view_direction: Vector3(1.0, 0.0, 0.0),
        horizontal_fov: PI / 4.0,
        vertical_fov: PI / 4.0,
        ray_bounces: 2,
        resolution: (256, 256),
        scene: scene,
        filename: filename.to_owned(),
    };

    camera.render();
}
