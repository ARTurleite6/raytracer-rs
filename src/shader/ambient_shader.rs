use crate::{
    helpers::{mul_vec3_with_rgb, Vec3},
    light::Light,
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
    fn shade(&self, intersection: &Option<Intersection>, scene: &Scene) -> Color {
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
                        for light in scene.lights() {
                            match light {
                                Light::Ambient(ambient_light) => {
                                    color +=
                                        mul_vec3_with_rgb(Vec3::from(ambient), ambient_light.l());
                                }
                                _ => {}
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