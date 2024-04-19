use crate::{helpers::Color, light::Light, object::intersection::Intersection, scene::Scene};

pub mod ambient_shader;
pub mod whitted_shader;

pub trait Shader {
    fn shade(
        &self,
        intersection: &Option<Intersection>,
        scene: &Scene,
        depth: Option<u32>,
    ) -> Color;
}
