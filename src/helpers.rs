pub type Vec3 = nalgebra::Vector3<f64>;
pub type Mat3 = nalgebra::Matrix3<f64>;
pub type Color = Vec3;

pub fn face_forward(v1: Vec3, v2: Vec3) -> Vec3 {
    if v1.dot(&v2) < 0.0 {
        -1.0 * v1
    } else {
        v1
    }
}

pub trait Zeroable {
    fn is_zero(&self) -> bool;
}

impl Zeroable for [f32; 3] {
    fn is_zero(&self) -> bool {
        *self == [0.0, 0.0, 0.0]
    }
}

pub fn mul_vec3_with_rgb(v: Vec3, c: Vec3) -> Vec3 {
    Vec3::new(v.x * c.x, v.y * c.y, v.z * c.z)
}

pub trait Comparable {
    fn min_between(&self, other: &Self) -> Self;
    fn max_between(&self, other: &Self) -> Self;
}

impl Comparable for Vec3 {
    fn min_between(&self, other: &Self) -> Self {
        Vec3::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    fn max_between(&self, other: &Self) -> Self {
        Vec3::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }
}
