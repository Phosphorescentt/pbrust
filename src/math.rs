use std::ops::Add;

#[derive(Debug, Default)]
pub struct Vector2(pub f32, pub f32);

#[derive(Debug, Default)]
pub struct Vector3(pub f32, pub f32, pub f32);

#[derive(Debug, Default)]
pub struct Vector4(pub f32, pub f32, pub f32, pub f32);

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Vector3) -> Self::Output {
        return Vector3(self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }
}
