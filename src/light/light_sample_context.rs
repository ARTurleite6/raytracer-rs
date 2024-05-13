use crate::{helpers::Vec3, object::intersection::Intersection};

#[derive(Debug)]
pub struct LightSampleContext {
    pub intersection_point: Vec3,
    pub geometric_normal: Vec3,
    shading_normal: Vec3,
}

impl LightSampleContext {
    pub fn new(intersection: &Intersection) -> Self {
        Self {
            intersection_point: intersection.point(),
            geometric_normal: intersection.geometric_normal(),
            shading_normal: intersection.shading_normal(),
        }
    }
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
