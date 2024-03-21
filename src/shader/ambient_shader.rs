use crate::object::intersection::Intersection;

use super::{Color, Shader};

pub struct AmbientShader {
    background_color: Color,
}

impl AmbientShader {
    pub fn new(background_color: Color) -> Self {
        Self { background_color }
    }
}

impl Shader for AmbientShader {
    fn shade(&self, _intersection: &Intersection) -> Color {
        todo!("Implement shade")
    }
}
