pub mod ambient_light;
pub mod area_light;
pub mod light_sample_context;
pub mod light_sampler;
pub mod point_light;

use rand::Rng;
use serde::Deserialize;

use crate::helpers::{Color, Vec2, Vec3};

use self::ambient_light::AmbientLight;
use self::area_light::{AreaLight, AreaLightArgs};
use self::point_light::PointLight;

#[derive(Default)]
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

#[derive(Debug, Clone)]
pub enum Light {
    Ambient(AmbientLight),
    Point(PointLight),
    Area(AreaLight),
}

impl Light {
    pub fn is_ambient_light(&self) -> bool {
        match self {
            Self::Ambient(_) => true,
            _ => false,
        }
    }

    pub fn l(&self) -> SampleLightResult {
        match self {
            Self::Area(area_light) => {
                let mut rng = rand::thread_rng();
                let randoms = Vec2::new(rng.gen(), rng.gen());
                area_light.l(&randoms)
            }
            Self::Point(point_light) => {
                let (color, point) = point_light.l();
                SampleLightResult {
                    color,
                    point: point.into(),
                    ..Default::default()
                }
            }
            Self::Ambient(ambient_light) => {
                let color = ambient_light.l();
                SampleLightResult {
                    color,
                    ..Default::default()
                }
            }
        }
    }

    pub fn intensity(&self) -> Color {
        match self {
            Self::Ambient(light) => light.l(),
            Self::Point(light) => light.l().0,
            Self::Area(light) => *light.intensity(),
        }
    }
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
