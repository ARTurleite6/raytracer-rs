use crate::{helpers::Color, light::Light, object::intersection::Intersection};

pub mod ambient_shader;

pub trait Shader {
    fn shade(&self, intersection: &Option<Intersection>, lights: &[Light]) -> Color;
}
