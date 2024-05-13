use rand::distributions::{Distribution, WeightedIndex};

use crate::{
    helpers::{Color, Vec3},
    light::{
        ambient_light::AmbientLight, area_light::AreaLight,
        light_sample_context::LightSampleContext, Light,
    },
};

use super::{HasBaseSampler, LightSampler, SampleLight};

#[derive(Debug)]
pub struct BaseSampler {
    pub(super) ambient_lights: Vec<AmbientLight>,
    pub(super) positional_lights: Vec<Light>,
    pub(super) weights: Vec<f64>,
}

impl BaseSampler {
    pub fn new(lights: Vec<Light>) -> Self {
        let (ambient_lights, positional_lights): (Vec<Light>, Vec<Light>) =
            lights.into_iter().partition(Light::is_ambient_light);

        let ambient_lights = ambient_lights
            .into_iter()
            .map(|light| {
                let Light::Ambient(light) = light else {
                    unreachable!("this has to be a ambient light as checked above");
                };
                light
            })
            .collect();

        let weights = positional_lights.iter().map(|_| 1.).collect();

        Self {
            ambient_lights,
            positional_lights,
            weights,
        }
    }
}

impl LightSampler for BaseSampler {
    fn geometric_lights(&self) -> Vec<AreaLight> {
        self.positional_lights
            .iter()
            .filter_map(|light| {
                if let Light::Area(light) = light {
                    Some(light.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    fn sample_ambient_lights(&self, ambient_component: [f32; 3]) -> Color {
        let ambient = [
            ambient_component[0] as f64,
            ambient_component[1] as f64,
            ambient_component[2] as f64,
        ];

        self.ambient_lights
            .iter()
            .map(|light| Vec3::from(ambient).component_mul(&light.l()))
            .sum()
    }

    fn sample(&self, _ctx: LightSampleContext) -> Option<SampleLight> {
        let mut rng = rand::thread_rng();
        let dist = WeightedIndex::new(&self.weights).ok()?;

        Some(SampleLight {
            light: self.positional_lights[dist.sample(&mut rng)].clone(),
            power: 1. / self.positional_lights.len() as f64,
        })
    }
}

impl HasBaseSampler for BaseSampler {
    fn base_sampler(&self) -> &BaseSampler {
        self
    }
}
