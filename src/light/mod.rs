pub mod ambient_light;
pub mod area_light;
pub mod light_sample_context;
pub mod light_sampler;
pub mod point_light;

use fastrand::Rng;
use serde::Deserialize;

use crate::helpers::{Color, Vec2, Vec3};
use crate::object::intersection::Intersection;

use self::ambient_light::AmbientLight;
use self::area_light::{AreaLight, AreaLightArgs};
use self::point_light::{PointLight, PointLightArgs};

#[derive(Default, Debug, Clone)]
pub struct SampleLightResult {
    pub power_gs: f64,
    pub color: Color,
    pub point: Option<Vec3>,
    pub pdf: Option<f64>,
    pub distance: Option<f64>,
    pub cos: Option<f64>,
    pub light_dir: Option<Vec3>,
}

impl SampleLightResult {
    pub fn calculate_data(mut self, light: &Light, intersection: &Intersection) -> Self {
        match light {
            Light::Point(_) => {
                let light_pos = self.point.unwrap();
                let mut light_dir = light_pos - intersection.point();
                let light_distance = light_dir.norm();
                light_dir.normalize_mut();

                self.cos = light_dir.dot(&intersection.shading_normal()).into();
                self.distance = light_distance.into();
                self.light_dir = light_dir.into();
            }
            Light::Area(light) => {
                let point = self.point.unwrap();
                let i_point = intersection.point();
                let mut light_dir = point - i_point;
                let light_distance = light_dir.norm();
                light_dir.normalize_mut();
                let cos_l = light_dir.dot(&intersection.shading_normal());

                let cos_l_la = light_dir.dot(light.normal());

                self.distance = light_distance.into();
                self.cos = if cos_l > 0.0 && cos_l_la <= 0.0 {
                    cos_l
                } else {
                    0.0
                }
                .into();
                self.light_dir = light_dir.into();
            }
            Light::Ambient(_) => {
                unreachable!("Calculate additional data is only needed for non ambient lights")
            }
        };

        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum LightArgs {
    Ambient(AmbientLight),
    Point(PointLightArgs),
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

    pub fn l(&self, rng: Option<&mut Rng>) -> SampleLightResult {
        match self {
            Self::Area(area_light) => {
                let rng = rng.unwrap();
                let randoms = Vec2::new(rng.f64(), rng.f64());
                area_light.l(&randoms)
            }
            Self::Point(point_light) => point_light.l(),
            Self::Ambient(ambient_light) => ambient_light.l(),
        }
    }
}

impl From<LightArgs> for Light {
    fn from(value: LightArgs) -> Self {
        match value {
            LightArgs::Point(point_light_args) => Light::Point(point_light_args.into()),
            LightArgs::Ambient(ambient_light) => Light::Ambient(ambient_light),
            LightArgs::Area(area_light_args) => Light::Area(area_light_args.into()),
        }
    }
}
