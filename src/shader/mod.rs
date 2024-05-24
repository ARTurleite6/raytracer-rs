use fastrand::Rng;

use crate::{
    helpers::Color, light::light_sampler::LightSampler, object::intersection::Intersection,
    scene::Scene,
};

pub mod ambient_shader;
pub mod better_path_tracer_shader;
pub mod distributed_shader;
pub mod path_tracer_shader;
pub mod whitted_shader;

pub trait Shader {
    fn shade<L: LightSampler>(
        &self,
        intersection: &Option<Intersection>,
        scene: &Scene,
        depth: Option<u32>,
        light_sampler: &L,
    ) -> Color;
}

pub trait BetterShader {
    fn shade<L: LightSampler>(
        &self,
        intersection: &Option<Intersection>,
        scene: &Scene,
        depth: Option<u32>,
        light_sampler: &L,
        rng: &mut Rng,
    ) -> Color;
}
