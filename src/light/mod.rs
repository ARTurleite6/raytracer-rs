pub mod ambient_light;
pub mod area_light;
pub mod point_light;

use serde::Deserialize;

use crate::helpers::{Color, Vec3};

use self::ambient_light::AmbientLight;
use self::area_light::{AreaLight, AreaLightArgs};
use self::point_light::PointLight;

pub struct SampleLightResult {
    pub color: Color,
    pub point: Option<Vec3>,
    pub pdf: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum LightArgs {
    Ambient(AmbientLight),
    Point(PointLight),
    Area(AreaLightArgs),
}

#[derive(Debug)]
pub enum Light {
    Ambient(AmbientLight),
    Point(PointLight),
    Area(AreaLight),
}

impl From<LightArgs> for Light {
    fn from(value: LightArgs) -> Self {
        match value {
            LightArgs::Point(point_light) => Light::Point(point_light),
            LightArgs::Ambient(ambient_light) => Light::Ambient(ambient_light),
            LightArgs::Area(area_light_args) => Light::Area(area_light_args.into()),
        }
    }
}
