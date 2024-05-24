use fastrand::Rng;
use rand::{
    distributions::{Distribution, WeightedIndex},
    rngs::StdRng,
    SeedableRng,
};

use crate::{
    helpers::{Color, Vec3},
    light::{
        ambient_light::AmbientLight, area_light::AreaLight,
        light_sample_context::LightSampleContext, Light,
    },
};

use super::{HasBaseSampler, LightSampler, SampleLight};

#[derive(Debug)]
pub struct BaseSampler<'a> {
    pub(super) ambient_lights: Vec<&'a AmbientLight>,
    pub(super) positional_lights: Vec<&'a Light>,
    pub(super) weights: Vec<f64>,
}

impl<'a> BaseSampler<'a> {
    pub fn new(lights: impl Iterator<Item = &'a Light>) -> Self {
        let (ambient_lights, positional_lights): (Vec<&Light>, Vec<&Light>) =
            lights.partition(|&light| light.is_ambient_light());

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

impl LightSampler for BaseSampler<'_> {
    fn geometric_lights(&self) -> impl Iterator<Item = &AreaLight> {
        self.positional_lights.iter().filter_map(|light| {
            if let Light::Area(light) = light {
                Some(light)
            } else {
                None
            }
        })
    }

    fn sample_ambient_lights(&self, ambient_component: [f32; 3]) -> Color {
        let ambient = [
            ambient_component[0] as f64,
            ambient_component[1] as f64,
            ambient_component[2] as f64,
        ];

        self.ambient_lights
            .iter()
            .map(|light| Vec3::from(ambient).component_mul(&light.l().color))
            .sum()
    }

    fn sample(&self, _ctx: LightSampleContext, rng: &mut Rng) -> Option<SampleLight> {
        let dist = WeightedIndex::new(&self.weights).ok()?;
        let mut std_rng = StdRng::from_entropy();

        let light = self.positional_lights[dist.sample(&mut std_rng)].clone();
        let sample_result = light.l(rng.into());
        Some(SampleLight {
            light,
            power: 1. / self.positional_lights.len() as f64,
            sample_result,
        })
    }
}

impl HasBaseSampler for BaseSampler<'_> {
    fn base_sampler(&self) -> &BaseSampler {
        self
    }
}
