use serde::Deserialize;

use crate::{
    helpers::{Color, Vec2, Vec3},
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
}

impl From<AreaLightArgs> for AreaLight {
    fn from(value: AreaLightArgs) -> Self {
        Self::new(value.vertex, value.power)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AreaLight {
    gem: Face,
    pdf: f64,
    intensity: Color,
    power: Color,
}

impl AreaLight {
    pub fn new(vertex: [Vec3; 3], power: Vec3) -> Self {
        let gem = FaceBuilder::new(vertex).build();
        let pdf = 1.0 / gem.area();
        let intensity = power * pdf;
        Self {
            gem,
            pdf,
            intensity,
            power,
        }
    }

    pub fn intensity(&self) -> &Color {
        &self.intensity
    }

    pub fn distance(&self, point: &Vec3) -> f64 {
        let vertices = self.gem.vertices();
        let normal = self.gem.normal();
        let d = -vertices[0].dot(&normal);
        (normal.dot(&point) + d).abs()
    }

    // TODO: Implement the cos of the angle
    //pub fn angle_cos(&self, normal: &Vec3) -> f64 {
    //    let vector_to_point = point - self.gem.vertices()[0];

    //    // Calculate the normal vector of the triangle
    //    let triangle_normal = self.normal();

    //    // Calculate the angle between the vector to the point and the triangle normal
    //    let angle = vector_to_point.dot(&triangle_normal)
    //        / (vector_to_point.norm() * triangle_normal.norm());
    //    angle
    //}

    pub fn normal(&self) -> Vec3 {
        self.gem.normal()
    }

    pub fn l(&self, randoms: Vec2) -> SampleLightResult {
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
            color: self.intensity,
            pdf: self.pdf.into(),
            point: point.into(),
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
