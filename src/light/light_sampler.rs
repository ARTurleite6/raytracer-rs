use rand::seq::IteratorRandom;

use crate::helpers::{Color, Vec3};

use super::{ambient_light::AmbientLight, light_sample_context::LightSampleContext, Light};

pub struct SampleLight {
    pub light: Light,
    pub power: f64,
}

pub trait LightSampler {
    fn sample_ambient_lights(&self, ambient_component: [f32; 3]) -> Color;

    fn sample(&self) -> Option<SampleLight>;
}

#[derive(Debug, Default)]
pub struct UniformLightSampler {
    ambient_lights: Vec<AmbientLight>,
    positional_lights: Vec<Light>,
}

impl UniformLightSampler {
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

        Self {
            ambient_lights,
            positional_lights,
        }
    }
}

impl LightSampler for UniformLightSampler {
    fn sample(&self) -> Option<SampleLight> {
        let mut rng = rand::thread_rng();
        Some(SampleLight {
            light: self.positional_lights.iter().choose(&mut rng)?.clone(),
            power: 1. / self.positional_lights.len() as f64,
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
            .map(|light| Vec3::from(ambient).component_mul(&light.l()))
            .sum()
    }
}
