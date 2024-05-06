pub mod ambient_light;
pub mod area_light;
pub mod point_light;

use serde::Deserialize;

use crate::helpers::{Color, Vec3};

use self::ambient_light::AmbientLight;
use self::area_light::AreaLight;
use self::point_light::PointLight;

pub struct SampleLightResult {
    pub color: Color,
    pub point: Option<Vec3>,
    pub pdf: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum Light {
    Ambient(AmbientLight),
    Point(PointLight),
    AreaLight(AreaLight),
}
