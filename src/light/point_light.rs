use serde::Deserialize;

use crate::helpers::{gray_scale, Color, Vec3};

use super::SampleLightResult;

#[derive(Debug, Clone, Deserialize)]
pub struct PointLightArgs {
    color: Color,
    pos: Vec3,
}

#[derive(Debug, Clone)]
pub struct PointLight {
    color: Color,
    pos: Vec3,
    power_gs: f64,
}

impl PointLight {
    pub fn l(&self) -> SampleLightResult {
        SampleLightResult {
            power_gs: self.power_gs,
            color: self.color,
            point: self.pos.into(),
            ..Default::default()
        }
    }
}

impl From<PointLightArgs> for PointLight {
    fn from(value: PointLightArgs) -> Self {
        Self {
            color: value.color,
            pos: value.pos,
            power_gs: gray_scale(&value.color),
        }
    }
}
