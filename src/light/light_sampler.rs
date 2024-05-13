use crate::helpers::Color;

use self::base_sampler::BaseSampler;

use super::{area_light::AreaLight, light_sample_context::LightSampleContext, Light};

mod base_sampler;
pub mod power_sampler;
pub mod uniform_sampler;

pub struct SampleLight {
    pub light: Light,
    pub power: f64,
}

pub trait LightSampler: HasBaseSampler {
    fn sample_ambient_lights(&self, ambient_component: [f32; 3]) -> Color {
        self.base_sampler().sample_ambient_lights(ambient_component)
    }

    fn geometric_lights(&self) -> Vec<AreaLight> {
        self.base_sampler().geometric_lights()
    }

    fn sample(&self, context: LightSampleContext) -> Option<SampleLight> {
        self.base_sampler().sample(context)
    }
}

pub trait HasBaseSampler {
    fn base_sampler(&self) -> &BaseSampler;
}
