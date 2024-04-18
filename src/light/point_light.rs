use crate::helpers::{Color, Vec3};

#[derive(Debug, Clone)]
pub struct PointLight {
    color: Color,
    pos: Vec3,
}

impl PointLight {
    pub fn new(color: Color, pos: Vec3) -> Self {
        Self { color, pos }
    }

    pub fn l(&self) -> (Color, Vec3) {
        (self.color, self.pos)
    }
}
