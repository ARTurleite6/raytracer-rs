use serde::Deserialize;

use crate::helpers::Vec3;

use super::SampleLightResult;

#[derive(Debug, Clone, Deserialize)]
pub struct AmbientLight {
    color: Vec3,
}

impl AmbientLight {
    pub fn l(&self) -> SampleLightResult {
        SampleLightResult {
            color: self.color,
            ..Default::default()
        }
    }
}
