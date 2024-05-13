use nalgebra::Matrix3;
use tobj::Material;

pub type Vec3 = nalgebra::Vector3<f64>;
pub type Vec2 = nalgebra::Vector2<f64>;
pub type Mat3 = nalgebra::Matrix3<f64>;
pub type Color = Vec3;

pub trait CoordinateSystemProvider {
    fn coordinate_system(&self) -> (Vec3, Vec3);
}

pub trait Rotateable {
    fn rotate(&self, rx: Vec3, ry: Vec3, rz: Vec3) -> Self;
    fn face_forward(&self, wo: Vec3) -> Self;
}

impl Rotateable for Vec3 {
    fn rotate(&self, rx: Vec3, ry: Vec3, rz: Vec3) -> Self {
        let rotation_matrix = Matrix3::new(rx.x, ry.x, rz.x, rx.y, ry.y, rz.y, rx.z, ry.z, rz.z);
        rotation_matrix * self
    }

    fn face_forward(&self, wo: Vec3) -> Self {
        if self.dot(&wo) < 0.0 {
            -1. * self
        } else {
            *self
        }
    }
}

impl CoordinateSystemProvider for Vec3 {
    fn coordinate_system(&self) -> (Vec3, Vec3) {
        let v2 = if self.x.abs() > self.y.abs() {
            Vec3::new(-self.z, 0.0, self.x) / (self.x * self.x + self.z * self.z).sqrt()
        } else {
            Vec3::new(0.0, self.z, -self.y) / (self.y * self.y + self.z * self.z).sqrt()
        };
        let v3 = self.cross(&v2);
        (v2, v3)
    }
}

pub fn has_component<F, Component>(material: &Material, component_getter: F) -> bool
where
    Component: Zeroable,
    F: FnOnce(&Material) -> Option<Component>,
{
    let component = component_getter(material);

    component.is_some_and(|value| !value.is_zero())
}

pub trait Zeroable {
    fn is_zero(&self) -> bool;
}

impl Zeroable for [f32; 3] {
    fn is_zero(&self) -> bool {
        *self == [0.0, 0.0, 0.0]
    }
}

pub fn gray_scale(color: &Color) -> f64 {
    0.299 * color.x + 0.587 * color.y + 0.114 * color.z
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
