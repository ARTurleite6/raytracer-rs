use nalgebra::ComplexField;
use rand::distributions::{Distribution, WeightedIndex};

use crate::{
    helpers::gray_scale,
    light::{light_sample_context::LightSampleContext, Light},
};

use super::{base_sampler::BaseSampler, HasBaseSampler, LightSampler, SampleLight};

#[derive(Debug)]
pub struct PowerLightSampler {
    base_sampler: BaseSampler,
}

impl PowerLightSampler {
    #[allow(dead_code)]
    pub fn new(lights: Vec<Light>) -> Self {
        Self {
            base_sampler: BaseSampler::new(lights),
        }
    }
}

impl LightSampler for PowerLightSampler {
    fn sample(&self, context: LightSampleContext) -> Option<SampleLight> {
        let weights = self.base_sampler.positional_lights.iter().map(|light| {
            let (power, distance) = match light {
                Light::Area(area_light) => (
                    *area_light.intensity(),
                    area_light.distance(&context.intersection_point),
                ),
                Light::Point(point_light) => (
                    point_light.l().0,
                    point_light.distance(&context.intersection_point),
                ),
                _ => {
                    unreachable!("this case cannot happen")
                }
            };
            gray_scale(&power) / distance.powi(2)
        });

        let dist = WeightedIndex::new(weights).ok()?;
        let mut rng = rand::thread_rng();

        Some(SampleLight {
            light: self.base_sampler.positional_lights[dist.sample(&mut rng)].clone(),
            power: 1. / self.base_sampler.positional_lights.len() as f64,
        })
    }
}

impl HasBaseSampler for PowerLightSampler {
    fn base_sampler(&self) -> &BaseSampler {
        &self.base_sampler
    }
}
