use crate::{helpers::Vec3, object::intersection::Intersection};

#[derive(Debug)]
pub struct LightSampleContext {
    intersection_point: Vec3,
    geometric_normal: Vec3,
    shading_normal: Vec3,
}

impl From<Intersection> for LightSampleContext {
    fn from(value: Intersection) -> Self {
        Self {
            intersection_point: value.point(),
            geometric_normal: value.geometric_normal(),
            shading_normal: value.shading_normal(),
        }
    }
}
