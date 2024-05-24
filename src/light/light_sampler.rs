use fastrand::Rng;

use crate::helpers::Color;

use self::base_sampler::BaseSampler;

use super::{
    area_light::AreaLight, light_sample_context::LightSampleContext, Light, SampleLightResult,
};

mod base_sampler;
mod cumulative_distribution;
pub mod power_sampler;
pub mod uniform_sampler;

pub struct SampleLight {
    pub light: Light,
    pub power: f64,
    pub sample_result: SampleLightResult,
}

pub trait LightSampler: HasBaseSampler {
    fn sample_ambient_lights(&self, ambient_component: [f32; 3]) -> Color {
        self.base_sampler().sample_ambient_lights(ambient_component)
    }

    fn geometric_lights(&self) -> Vec<AreaLight> {
        self.base_sampler().geometric_lights()
    }

    fn sample(&self, context: LightSampleContext, rng: &mut Rng) -> Option<SampleLight> {
        self.base_sampler().sample(context, rng)
    }
}

pub trait HasBaseSampler {
    fn base_sampler(&self) -> &BaseSampler;
}
