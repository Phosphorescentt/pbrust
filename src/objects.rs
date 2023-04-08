pub use crate::math::{Vector3, Vector4};
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
        // let mut buffer: Vec<u8> =
        //     vec![0; (self.resolution.0 as usize) * (self.resolution.1 as usize) * 3];
        let mut buffer: Vec<u8> = Vec::new();

        for y in (0..self.resolution.1).rev() {
            for x in 0..self.resolution.0 {
                // let current_ray = Ray {
                //     start_position: self.position,
                //     direction: Vector3(0.0, 0.0, 0.0), // this is the trig bit
                //     max_steps: 1000,
                //     step_distance: 0.1,
                // };
                let r = x as f64 / (self.resolution.0) as f64;
                let g = y as f64 / (self.resolution.1) as f64;
                let b = 0.25;

                let ir = (255.0 * r) as u8;
                let ig = (255.0 * g) as u8;
                let ib = (255.0 * b) as u8;

                buffer.push(ir);
                buffer.push(ig);
                buffer.push(ib);
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

        // for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        //     let r = x as f64 / (self.resolution.0) as f64;
        //     let g = y as f64 / (self.resolution.1) as f64;
        //     let b = 0.25;
        //
        //     let ir = (255.0 * r) as u8;
        //     let ig = (255.0 * g) as u8;
        //     let ib = (255.0 * b) as u8;
        //
        //     *pixel = Rgb([ir, ig, ib])
        // }

        // buffer.save(Path::new(&self.filename)).unwrap();
        // match buffer.save(&self.filename) {
        //     Ok(_) => println!("Good"),
        //     Err(e) => println!("{}", e),
        // };
    }
}

impl Renderable for Sphere {
    fn intersection(&self, start: Vector3, direction: Vector3) -> Option<(Vector3, Colour)> {
        return Some((Vector3::default(), Colour(255, 255, 255)));
    }
}

impl Ray {
    fn find_intersect(&self, object: Sphere) -> Colour {
        return Colour(0, 0, 0);
    }
}

pub trait LightSource {}
