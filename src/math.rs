use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug, Default)]
pub struct Vector2(pub f32, pub f32);

#[derive(Copy, Clone, Debug, Default)]
pub struct Vector3(pub f32, pub f32, pub f32);

#[derive(Copy, Clone, Debug, Default)]
pub struct Mat3 {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
    pub m20: f32,
    pub m21: f32,
    pub m22: f32,
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Vector3) -> Self::Output {
        return Vector3(self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Vector3) -> Self::Output {
        return Vector3(self.0 - other.0, self.1 - other.1, self.2 - other.2);
    }
}

impl Vector3 {
    pub fn abs(self) -> f32 {
        return self.absp2().sqrt();
    }

    pub fn absp2(self) -> f32 {
        return self.0 * self.0 + self.1 * self.1 + self.2 * self.2;
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f32) -> Self::Output {
        let mid = Vector3(self.0 * other, self.1 * other, self.2 * other);
        return mid;
    }
}

impl Mul<Mat3> for Mat3 {
    type Output = Self;

    fn mul(self, other: Mat3) -> Self::Output {
        return Mat3 {
            m00: self.m00 * other.m00 + self.m01 * other.m10 + self.m02 * other.m20,
            m01: self.m00 * other.m01 + self.m01 * other.m11 + self.m02 * other.m21,
            m02: self.m00 * other.m02 + self.m01 * other.m12 + self.m02 * other.m22,
            m10: self.m10 * other.m00 + self.m11 * other.m10 + self.m12 * other.m20,
            m11: self.m10 * other.m01 + self.m11 * other.m11 + self.m12 * other.m21,
            m12: self.m10 * other.m02 + self.m11 * other.m12 + self.m12 * other.m22,
            m20: self.m20 * other.m00 + self.m21 * other.m10 + self.m22 * other.m20,
            m21: self.m20 * other.m01 + self.m21 * other.m11 + self.m22 * other.m21,
            m22: self.m20 * other.m02 + self.m21 * other.m12 + self.m22 * other.m22,
        };
    }
}

impl Mul<Vector3> for Mat3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Self::Output {
        return Vector3(
            self.m00 * other.0 + self.m01 * other.1 + self.m02 * other.2,
            self.m10 * other.0 + self.m11 * other.1 + self.m12 * other.2,
            self.m20 * other.0 + self.m21 * other.1 + self.m22 * other.1,
        );
    }
}
