use crate::light::Light;

use super::{base_sampler::BaseSampler, HasBaseSampler, LightSampler};

#[derive(Debug)]
pub struct UniformLightSampler {
    base_sampler: BaseSampler,
}

impl UniformLightSampler {
    pub fn new(lights: Vec<Light>) -> Self {
        Self {
            base_sampler: BaseSampler::new(lights),
        }
    }
}

impl LightSampler for UniformLightSampler {}

impl HasBaseSampler for UniformLightSampler {
    fn base_sampler(&self) -> &BaseSampler {
        &self.base_sampler
    }
}
