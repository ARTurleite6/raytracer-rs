use serde::Deserialize;

use crate::helpers::Vec3;

#[derive(Debug, Clone, Deserialize)]
pub struct AmbientLight {
    color: Vec3,
}

impl AmbientLight {
    pub fn l(&self) -> Vec3 {
        self.color
    }
}
