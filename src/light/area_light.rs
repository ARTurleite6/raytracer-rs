use serde::Deserialize;

use crate::{
    helpers::{gray_scale, Color, Vec2, Vec3},
    object::{
        face::{Face, FaceBuilder},
        intersection::Intersectable,
    },
};

use super::SampleLightResult;

#[derive(Debug, Deserialize)]
pub struct AreaLightArgs {
    vertex: [Vec3; 3],
    power: Color,
    normal: Vec3,
}

impl From<AreaLightArgs> for AreaLight {
    fn from(value: AreaLightArgs) -> Self {
        Self::new(value.vertex, &value.power, &value.normal)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AreaLight {
    gem: Face,
    pdf: f64,
    intensity: Color,
    power: Color,
    power_gs: f64,
}

impl AreaLight {
    pub fn new(vertex: [Vec3; 3], power: &Vec3, normal: &Vec3) -> Self {
        let gem = FaceBuilder::new(vertex).normal(normal).build();
        let pdf = 1.0 / gem.area();
        let intensity = power * pdf;
        let power_gs = gray_scale(&intensity);
        Self {
            gem,
            pdf,
            intensity,
            power: *power,
            power_gs,
        }
    }

    pub fn normal(&self) -> &Vec3 {
        self.gem.normal()
    }

    pub fn l(&self, randoms: &Vec2) -> SampleLightResult {
        let sqrt_r0 = randoms.x.sqrt();
        let alpha = 1.0 - sqrt_r0;
        let beta = (1.0 - randoms.y) * sqrt_r0;
        let gamma = randoms.y * sqrt_r0;

        let vertices = self.gem.vertices();

        let point = Vec3::new(
            alpha * vertices[0].x + beta * vertices[1].x + gamma * vertices[2].x,
            alpha * vertices[0].y + beta * vertices[1].y + gamma * vertices[2].y,
            alpha * vertices[0].z + beta * vertices[1].z + gamma * vertices[2].z,
        );

        SampleLightResult {
            power_gs: self.power_gs,
            color: self.intensity,
            pdf: self.pdf.into(),
            point: point.into(),
            ..Default::default()
        }
    }
}

impl Intersectable for AreaLight {
    fn intersect(
        &self,
        ray: &crate::object::ray::Ray,
    ) -> Option<crate::object::intersection::Intersection> {
        let mut intersection = self.gem.intersect(ray)?;

        intersection.light_intensity = Some(self.power);

        Some(intersection)
    }
}
