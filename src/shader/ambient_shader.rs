use crate::{
    helpers::{mul_vec3_with_rgb, Vec3, Zeroable},
    light::{ambient_light, Light},
    object::intersection::Intersection,
    scene::Scene,
};

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
    fn shade(
        &self,
        intersection: &Option<Intersection>,
        scene: &Scene,
        _depth: Option<u32>,
    ) -> Color {
        let mut color = Vec3::new(0.0, 0.0, 0.0);

        match intersection {
            Some(intersection) => {
                let material = intersection
                    .brdf
                    .as_ref()
                    .expect("Expected a material")
                    .material
                    .as_ref()
                    .expect("Expected a material object in info");

                match material.ambient {
                    Some(ambient) => {
                        if !ambient.is_zero() {
                            for light in scene.lights() {
                                if let Light::Ambient(ambient_light) = light {
                                    let ambient =
                                        [ambient[0] as f64, ambient[1] as f64, ambient[2] as f64];
                                    color +=
                                        mul_vec3_with_rgb(Vec3::from(ambient), ambient_light.l());
                                }
                            }
                        }
                        color
                    }
                    None => color,
                }
            }
            None => self.background_color,
        }
    }
}
