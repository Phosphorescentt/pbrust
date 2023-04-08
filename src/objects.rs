use std::ops::Mul;

pub use crate::math::{Mat3, Vector3};
use image::ColorType;

trait Renderable {
    fn test_inside(&self, point: Vector3) -> bool;
    fn find_intersection(&self, p: Vector3, q: Vector3) -> Vector3;
    fn normal_to_surface_at(&self, p: Vector3) -> Vector3;
}

#[derive(Copy, Clone)]
pub struct Colour(pub f32, pub f32, pub f32);
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
    pub colour: Colour,
}

pub struct Ray {
    pub start_position: Vector3,
    pub current_position: Vector3,
    pub previous_position: Vector3,
    pub direction: Vector3,
    pub max_steps: u32,
    pub step_size: f32,
    pub max_bounces: u8,
}

pub struct Scene {
    pub objects: Vec<Sphere>,
}

pub struct Camera {
    pub position: Vector3,
    pub view_direction: Vector3,
    pub horizontal_fov: f32,
    pub vertical_fov: f32,
    pub ray_bounces: u8,
    pub resolution: (u32, u32),
    pub scene: Scene,
    pub filename: String,
}

impl Mul for Colour {
    type Output = Self;

    fn mul(self, other: Colour) -> Self::Output {
        return Colour(self.0 * other.0, self.1 * other.1, self.2 * other.2);
    }
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

                let mut current_ray = Ray {
                    start_position: self.position,
                    current_position: self.position,
                    previous_position: self.position,
                    direction: ray_direction,
                    max_steps: 1000,
                    step_size: 0.1,
                    max_bounces: self.ray_bounces,
                };

                match current_ray.cast(&self.scene) {
                    Some(c) => {
                        buffer.push((c.0 * 255.0) as u8);
                        buffer.push((c.1 * 255.0) as u8);
                        buffer.push((c.2 * 255.0) as u8);
                    }
                    None => {
                        buffer.push(0);
                        buffer.push(0);
                        buffer.push(0);
                    }
                }
            }
        }

        // Save the image
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
    fn test_inside(&self, point: Vector3) -> bool {
        let delta = self.position - point;

        if delta.absp2() <= self.radius * self.radius {
            return true;
        } else {
            return false;
        }
    }

    fn find_intersection(&self, p: Vector3, q: Vector3) -> Vector3 {
        let a = p.0.powi(2) + p.1.powi(2) + p.2.powi(2) + q.0.powi(2) + q.1.powi(2) + q.2.powi(2)
            - (p.0 * q.0 + p.1 * q.1 + p.2 * q.2);
        let b = p.0 * q.0 + p.1 * q.1 + p.2 * q.2 - 2.0 * (p.0.powi(2) + p.1.powi(2) + p.2.powi(2));
        let c = p.0.powi(2) + p.1.powi(2) + p.2.powi(2) - self.radius.powi(2);

        // TODO: fix this. something in here is very wrong

        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            println!("{}", discriminant);
            panic!("Discriminant < 0.0 lollers");
        } else if discriminant == 0.0 {
            // one solution
            let t = -b / (4.0 * a * c);
            let intersection_pos = p + (q - p) * t;
            return intersection_pos;
        } else {
            // two solutions, but return the one closer to the camera.
            // this should be the smaller of the two solutions
            let t1 = (-b - (b.powi(2) - 4.0 * a * c).sqrt()) / (4.0 * a); // so this one (hopefully)
            let t2 = (-b + (b.powi(2) - 4.0 * a * c).sqrt()) / (4.0 * a);

            if t1 <= t2 {
                let intersection_pos = p + (q - p) * t1;
                return intersection_pos;
            } else {
                let intersection_pos = p + (q - p) * t2;
                return intersection_pos;
            }
        }
    }

    fn normal_to_surface_at(&self, p: Vector3) -> Vector3 {
        let dir = p - self.position;
        return dir.normalised();
    }
}

impl Ray {
    pub fn cast(&mut self, scene: &Scene) -> Option<Colour> {
        let mut bounces = 0;
        self.previous_position = self.start_position;
        self.current_position = self.start_position;
        let mut current_colour = Colour(1.0, 1.0, 1.0);
        for _ in 0..self.max_steps {
            for object in scene.objects.iter().as_ref() {
                if object.test_inside(self.current_position) {
                    let intersection_point =
                        object.find_intersection(self.previous_position, self.current_position);

                    // change direction to the perfect reflection i guess
                    // so some trig (as per).
                    self.bounce(intersection_point, object);
                    bounces += 1;

                    current_colour = current_colour * object.material.colour;

                    if bounces > self.max_bounces {
                        return Some(current_colour);
                    }
                } else {
                    // do other stuff
                }
            }
            self.previous_position = self.current_position;
            self.current_position = self.current_position + self.direction * self.step_size;
        }
        return None;
    }

    pub fn bounce(&mut self, intersection_point: Vector3, object: &Sphere) -> () {
        // Normal specular reflection
        let normal = object.normal_to_surface_at(intersection_point);
        let delta = normal - (self.direction * -1.0);
        let new_direction = self.direction + delta * 2.0;
        self.current_position = intersection_point;
        self.direction = new_direction;
    }
}
