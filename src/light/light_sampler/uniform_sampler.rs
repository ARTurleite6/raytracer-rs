use crate::light::Light;

use super::{base_sampler::BaseSampler, HasBaseSampler, LightSampler};

#[derive(Debug)]
pub struct UniformLightSampler<'lights> {
    base_sampler: BaseSampler<'lights>,
}

impl<'lights> UniformLightSampler<'lights> {
    pub fn new(lights: impl Iterator<Item = &'lights Light>) -> Self {
        Self {
            base_sampler: BaseSampler::new(lights),
        }
    }
}

impl LightSampler for UniformLightSampler<'_> {}

impl HasBaseSampler for UniformLightSampler<'_> {
    fn base_sampler(&self) -> &BaseSampler {
        &self.base_sampler
    }
}
