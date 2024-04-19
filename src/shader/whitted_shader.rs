use tobj::Material;

use crate::helpers::{face_forward, mul_vec3_with_rgb, Vec3};
use crate::object::ray::Ray;
use crate::scene::Scene;
use crate::{helpers::Color, light::Light, object::intersection::Intersection, shader::Shader};

pub struct WhittedShader {
    background: Color,
}

impl WhittedShader {
    pub fn new(background: Color) -> Self {
        Self { background }
    }

    fn direct_lighting(
        &self,
        intersection: &Intersection,
        brdf: &Material,
        scene: &Scene,
    ) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);

        for light in scene.lights() {
            match light {
                Light::Ambient(ambient_light) => {
                    if let Some(ambient) = brdf.ambient {
                        color += mul_vec3_with_rgb(Vec3::from(ambient), ambient_light.l());
                    }
                }
                Light::Point(point_light) => {
                    if let Some(diffuse) = brdf.diffuse {
                        let (light_color, light_pos) = point_light.l();
                        let mut light_dir = light_pos - intersection.point();
                        let light_distance = light_dir.norm();
                        light_dir.normalize_mut();

                        let cos = light_dir.dot(&intersection.shading_normal());

                        if cos > 0.0 {
                            let mut shadow = Ray::new(intersection.point(), light_dir);
                            shadow.adjust_origin(intersection.geometric_normal());
                            if scene.visibility(&shadow, light_distance) {
                                color += mul_vec3_with_rgb(Vec3::from(diffuse), light_color) * cos;
                            }
                        }
                    }
                }
            }
        }

        color
    }
}

impl Shader for WhittedShader {
    fn shade(&self, intersection: &Option<Intersection>, scene: &Scene) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);

        let Some(intersection) = intersection else {
            return self.background;
        };

        let material = intersection
            .brdf
            .as_ref()
            .expect("BRDF in the intesection")
            .material
            .as_ref()
            .expect("Material in the material information");

        color += self.direct_lighting(&intersection, material, scene);

        color
    }
}
