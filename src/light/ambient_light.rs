use crate::helpers::Vec3;

#[derive(Debug, Clone)]
pub struct AmbientLight {
    color: Vec3,
}

impl AmbientLight {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }

    pub fn l(&self) -> Vec3 {
        self.color
    }
}
