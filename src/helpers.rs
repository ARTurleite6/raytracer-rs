pub type Vec3 = nalgebra::Vector3<f32>;
pub type Vec2 = nalgebra::Vector2<f32>;
pub type Mat3 = nalgebra::Matrix3<f32>;
pub type Color = Vec3;

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
