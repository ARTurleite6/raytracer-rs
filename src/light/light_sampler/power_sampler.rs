use fastrand::Rng;

use crate::light::{light_sample_context::LightSampleContext, Light};

use super::{
    base_sampler::BaseSampler, cumulative_distribution::CDF, HasBaseSampler, LightSampler,
    SampleLight,
};

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
    fn sample(&self, context: LightSampleContext, rng: &mut Rng) -> Option<SampleLight> {
        let mut weights: Vec<_> = self
            .base_sampler
            .positional_lights
            .iter()
            .map(|light| {
                let sample = light
                    .l(rng.into())
                    .calculate_data(&light, context.intersection);
                let cos = sample.cos.unwrap();
                let distance = sample.distance.unwrap();
                let power_gs = sample.power_gs;
                (sample, power_gs / distance.powi(2) * cos)
            })
            .collect();

        let total_weight: f64 = weights.iter().map(|w| w.1).sum();
        if total_weight > 0.0 {
            weights.iter_mut().for_each(|weight| {
                weight.1 /= total_weight;
            });
        }

        let dist = CDF::new(&weights);
        let (index, (sample_result, weight)) = dist.sample(rng)?;
        let light = self.base_sampler.positional_lights[index].clone();
        let power = weight;

        Some(SampleLight {
            light,
            power: *power,
            sample_result: sample_result.clone(),
        })
    }
}

impl HasBaseSampler for PowerLightSampler {
    fn base_sampler(&self) -> &BaseSampler {
        &self.base_sampler
    }
}
