use serde::Deserialize;

use crate::helpers::{Color, Vec3};

#[derive(Debug, Clone, Deserialize)]
pub struct PointLight {
    color: Color,
    pos: Vec3,
}

impl PointLight {
    pub fn distance(&self, point: &Vec3) -> f64 {
        (self.pos - point).norm()
    }

    pub fn l(&self) -> (Color, Vec3) {
        (self.color, self.pos)
    }
}
