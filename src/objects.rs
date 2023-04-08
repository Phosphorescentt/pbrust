use std::path::Path;

pub use crate::math::{Mat3, Vector3};
use image::{save_buffer, ColorType};

trait Renderable {
    fn intersection(&self, start: Vector3, direction: Vector3) -> Option<(Vector3, Colour)>;
}

pub struct Colour(pub u8, pub u8, pub u8);
pub struct ColourAlpha(pub u8, pub u8, pub u8, pub u8);

pub struct Material {
    pub colour: Colour,
}

pub struct Sphere {
    pub position: Vector3,
    pub radius: f32,
    pub material: Material,
}

pub struct SphereLight {
    pub shape: Sphere,
    pub material: Material,
}

pub struct Ray {
    pub start_position: Vector3,
    pub direction: Vector3,
    pub max_steps: i32,
    pub step_distance: f32,
}

pub struct Scene {
    pub objects: Vec<Sphere>,
}

pub struct Camera {
    pub position: Vector3,
    pub view_direction: Vector3,
    pub horizontal_fov: f32,
    pub vertical_fov: f32,
    pub resolution: (u32, u32),
    pub scene: Scene,
    pub filename: String,
}

impl Camera {
    pub fn render(&self) {
        let mut buffer: Vec<u8> = Vec::new();
        for y in (0..self.resolution.1).rev() {
            for x in 0..self.resolution.0 {
                let theta = (self.horizontal_fov / (self.resolution.0 as f32)) * (x as f32)
                    - self.horizontal_fov / 2.0;

                let phi = (self.vertical_fov / (self.resolution.1 as f32)) * (y as f32)
                    - self.vertical_fov / 2.0;

                let y_rotation_matrix = Mat3 {
                    m00: theta.cos(),
                    m02: theta.sin(),
                    m11: 1.0,
                    m20: -theta.sin(),
                    m22: theta.cos(),
                    ..Default::default()
                };

                let z_rotation_matrix = Mat3 {
                    m00: phi.cos(),
                    m01: -phi.sin(),
                    m10: phi.sin(),
                    m11: phi.cos(),
                    m22: 1.0,
                    ..Default::default()
                };

                let full_rotation_matrix = z_rotation_matrix * y_rotation_matrix;
                let ray_direction = full_rotation_matrix * Vector3(1.0, 0.0, 0.0);

                let current_ray = Ray {
                    start_position: self.position,
                    direction: ray_direction,
                    max_steps: 1000,
                    step_distance: 0.1,
                };

                let mut intersected = false;
                for object in &self.scene.objects {
                    match current_ray.find_intersect(object) {
                        Some(c) => {
                            buffer.push(c.0);
                            buffer.push(c.1);
                            buffer.push(c.2);
                            intersected = true;
                            break;
                        }
                        None => { /* continue */ }
                    }
                }

                // If we don't intersect anything then just make the pixel black.
                if !intersected {
                    buffer.push(0);
                    buffer.push(0);
                    buffer.push(0);
                }
            }
        }

        match image::save_buffer(
            &self.filename,
            &buffer,
            self.resolution.0,
            self.resolution.1,
            ColorType::Rgb8,
        ) {
            Ok(_) => println!("Image saved successfully"),
            Err(e) => eprintln!("{}", e),
        }
    }
}

impl Renderable for Sphere {
    fn intersection(&self, start: Vector3, direction: Vector3) -> Option<(Vector3, Colour)> {
        return Some((Vector3::default(), Colour(255, 255, 255)));
    }
}

impl Ray {
    pub fn find_intersect(&self, object: &Sphere) -> Option<Colour> {
        // if self.direction.0 > 0.1 {
        //     return Some(Colour(255, 0, 0));
        // }
        if self.direction.1 > 0.1 {
            return Some(Colour(0, 255, 0));
        }
        if self.direction.2 > 0.1 {
            return Some(Colour(0, 0, 255));
        }
        return Some(Colour(0, 0, 0));
    }
}
