use crate::light::{light_sample_context::LightSampleContext, Light};

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
    fn sample(&self, _context: LightSampleContext) -> Option<SampleLight> {
        todo!("falta implementar isto");
        //let weights = self.base_sampler.positional_lights.iter().map(|light| {
        //    let result = light.l();

        //    match light {
        //        Light::Area(area) => {}
        //    }
        //});

        //let dist = WeightedIndex::new(weights).ok()?;
        //let mut rng = rand::thread_rng();

        //Some(SampleLight {
        //    light: self.base_sampler.positional_lights[dist.sample(&mut rng)].clone(),
        //    power: 1. / self.base_sampler.positional_lights.len() as f64,
        //})
    }
}

impl HasBaseSampler for PowerLightSampler {
    fn base_sampler(&self) -> &BaseSampler {
        &self.base_sampler
    }
}
