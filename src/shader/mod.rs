use fastrand::Rng;

use crate::{helpers::Color, object::intersection::Intersection, scene::Scene};

pub mod ambient_shader;
pub mod better_path_tracer_shader;
pub mod distributed_shader;
pub mod path_tracer_shader;
pub mod whitted_shader;

pub trait Shader {
    fn shade(
        &self,
        intersection: &Option<Intersection>,
        scene: &Scene,
        depth: Option<u32>,
    ) -> Color;
}

pub trait BetterShader {
    fn shade(
        &self,
        intersection: &Option<Intersection>,
        scene: &Scene,
        depth: Option<u32>,
        rng: &mut Rng,
    ) -> Color;
}
