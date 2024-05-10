use rand::Rng;
use tobj::Material;

use crate::{
    helpers::{mul_vec3_with_rgb, Color, Vec2, Vec3, Zeroable},
    light::{Light, SampleLightResult},
    object::{intersection::Intersection, ray::Ray},
    scene::Scene,
};

use super::Shader;

pub struct DistributedShader {
    background: Color,
}

impl DistributedShader {
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
                Light::Area(area_light) => {
                    if let Some(diffuse) = brdf.diffuse {
                        if !diffuse.is_zero() {
                            let mut rng = rand::thread_rng();
                            let rnd = Vec2::new(rng.gen(), rng.gen());

                            let SampleLightResult {
                                color: light_color,
                                point,
                                pdf,
                            } = area_light.l(rnd);
                            let point = point.unwrap();

                            let mut light_dir = point - intersection.point();
                            let light_distance = light_dir.norm();
                            light_dir.normalize_mut();

                            let cos_l = light_dir.dot(&intersection.shading_normal());
                            let cos_l_la = light_dir.dot(&area_light.normal());

                            if cos_l > 0.0 && cos_l_la <= 0.0 {
                                let mut shadow = Ray::new(intersection.point(), light_dir);
                                shadow.adjust_origin(intersection.geometric_normal());

                                if scene.visibility(&shadow, light_distance - 0.0001) {
                                    let diffuse =
                                        [diffuse[0] as f64, diffuse[1] as f64, diffuse[2] as f64];

                                    color += (mul_vec3_with_rgb(Vec3::from(diffuse), light_color)
                                        * cos_l)
                                        / pdf.unwrap();
                                }
                            }
                        }
                    }
                }
            }
        }

        color
    }
}

impl Shader for DistributedShader {
    fn shade(
        &self,
        intersection: &Option<Intersection>,
        scene: &Scene,
        depth: Option<u32>,
    ) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);

        let Some(intersection) = intersection else {
            return self.background;
        };

        if intersection.is_light() {
            return intersection.light_intensity.unwrap();
        }

        let material = intersection
            .brdf
            .as_ref()
            .expect("brdf in intersection")
            .material
            .as_ref()
            .expect("material in material information");

        let depth = depth.unwrap_or(0);
        if let Some(specular) = material.specular {
            if !specular.is_zero() && depth < 4 {
                color += self.specular_reflection(intersection, scene, depth + 1);
            }
        }

        if let Some(diffuse) = material.diffuse {
            if !diffuse.is_zero() {
                color += self.direct_lighting(intersection, material, scene);
            }
        }

        color
    }
}
