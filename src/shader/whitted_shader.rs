use tobj::Material;

use crate::helpers::{mul_vec3_with_rgb, Vec3, Zeroable};
use crate::object::ray::Ray;
use crate::scene::Scene;
use crate::{helpers::Color, light::Light, object::intersection::Intersection, shader::Shader};

pub struct WhittedShader {
    background: Color,
}

impl WhittedShader {
    #[allow(dead_code)]
    pub fn new(background: Color) -> Self {
        Self { background }
    }

    fn specular_reflection(&self, intersection: &Intersection, scene: &Scene, depth: u32) -> Color {
        let wo = intersection.w_outgoing();
        let gn = intersection.geometric_normal();

        let cos = gn.dot(&wo);

        let rdir = (2.0 * cos * gn) - wo;
        let mut specular = Ray::new(intersection.point(), rdir);
        specular.adjust_origin(gn);

        let intersection = scene.trace(&specular);

        self.shade(&intersection, scene, Some(depth + 1))
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
                        if !ambient.is_zero() {
                            let ambient = [ambient[0] as f64, ambient[1] as f64, ambient[2] as f64];
                            color += mul_vec3_with_rgb(Vec3::from(ambient), ambient_light.l());
                        }
                    }
                }
                Light::Point(point_light) => {
                    if let Some(diffuse) = brdf.diffuse {
                        if !diffuse.is_zero() {
                            let (light_color, light_pos) = point_light.l();
                            let mut light_dir = light_pos - intersection.point();
                            let light_distance = light_dir.norm();
                            light_dir.normalize_mut();

                            let cos = light_dir.dot(&intersection.shading_normal());

                            if cos > 0.0 {
                                let mut shadow = Ray::new(intersection.point(), light_dir);
                                shadow.adjust_origin(intersection.geometric_normal());
                                if scene.visibility(&shadow, light_distance) {
                                    let diffuse =
                                        [diffuse[0] as f64, diffuse[1] as f64, diffuse[2] as f64];
                                    color +=
                                        mul_vec3_with_rgb(Vec3::from(diffuse), light_color) * cos;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        color
    }
}

impl Shader for WhittedShader {
    fn shade(
        &self,
        intersection: &Option<Intersection>,
        scene: &Scene,
        depth: Option<u32>,
    ) -> Color {
        let depth = depth.unwrap_or(0);
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

        color += self.direct_lighting(intersection, material, scene);

        if let Some(specular_material) = material.specular {
            if !specular_material.is_zero() && depth < 3 {
                color += self.specular_reflection(intersection, scene, depth + 1);
            }
        }

        color
    }
}
