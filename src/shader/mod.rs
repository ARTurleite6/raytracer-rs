use crate::{helpers::Color, object::intersection::Intersection};

pub mod ambient_shader;

pub trait Shader {
    fn shade(&self, intersection: &Intersection) -> Color;
}
