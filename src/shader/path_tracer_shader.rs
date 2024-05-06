use rand::Rng;
use tobj::Material;

use crate::{
    helpers::{Color, CoordinateSystemProvider, Rotateable, Vec2, Vec3, Zeroable},
    light::{Light, SampleLightResult},
    object::{intersection::Intersection, ray::Ray},
    scene::Scene,
};

use std::f64::consts::PI;

use super::Shader;

type Brdf = [f32; 3];
const MAX_DEPTH: u32 = 2;

pub struct PathTracerShader {
    background: Color,
    continue_p: f64,
}

impl PathTracerShader {
    pub fn new(background: Color) -> Self {
        Self {
            background,
            continue_p: 0.5,
        }
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
                            color += Vec3::from(ambient).component_mul(&ambient_light.l());
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
                                    color += Vec3::from(diffuse).component_mul(&light_color) * cos;
                                }
                            }
                        }
                    }
                }
                Light::AreaLight(area_light) => {
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
                            let _i_point = intersection.point();

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

                                    color += (Vec3::from(diffuse).component_mul(&light_color)
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

    fn diffuse_reflection(
        &self,
        intersection: &Intersection,
        material: &Brdf,
        scene: &Scene,
        depth: u32,
    ) -> Color {
        let mut rng = rand::thread_rng();
        let randoms = [rng.gen::<f64>(), rng.gen::<f64>()];

        let sqrt_rand1 = randoms[1].sqrt();
        let d_around = Vec3::new(
            (2. * PI * randoms[0]).cos() * (1. - randoms[1]).sqrt(),
            (2. * PI * randoms[0]).sin() * (1. - randoms[1]).sqrt(),
            sqrt_rand1,
        );

        let cos_theta = sqrt_rand1;
        let pdf = cos_theta / PI;

        let gn = intersection.geometric_normal();
        let (rx, ry) = gn.coordinate_system();

        let diffuse =
            Ray::new_with_adjusted_origin(intersection.point(), d_around.rotate(rx, ry, gn), gn);
        let d_intersection = scene.trace(&diffuse);
        if let Some(d_intersection) = d_intersection {
            if !d_intersection.is_light() {
                let r_color = self.shade(&d_intersection.into(), scene, Some(depth + 1));

                return (Color::from_column_slice(&[
                    material[0] as f64,
                    material[1] as f64,
                    material[2] as f64,
                ]) * cos_theta)
                    .component_mul(&r_color)
                    / pdf;
            }
        }
        Color::default()
    }

    fn specular_reflection(
        &self,
        intersection: &Intersection,
        material: &Brdf,
        scene: &Scene,
        depth: u32,
    ) -> Color {
        let gn = intersection.geometric_normal();
        let wo = intersection.w_outgoing();

        let cos = gn.dot(&wo);

        let r_dir = 2.0 * cos * gn - wo;
        let specular = Ray::new_with_adjusted_origin(intersection.point(), r_dir, gn);

        let specular_intersection = scene.trace(&specular);
        let r_color = self.shade(&specular_intersection, scene, Some(depth + 1));

        Vec3::from_column_slice(&[material[0] as f64, material[1] as f64, material[2] as f64])
            .component_mul(&r_color)
    }
}

impl Shader for PathTracerShader {
    fn shade(
        &self,
        intersection: &Option<Intersection>,
        scene: &Scene,
        depth: Option<u32>,
    ) -> Color {
        let mut color = Color::default();
        let Some(intersection) = intersection else {
            return self.background;
        };
        let depth = depth.unwrap_or(0);

        if intersection.is_light() {
            return intersection.light_intensity.unwrap();
        }

        let material = intersection
            .brdf
            .as_ref()
            .expect("Expected a material")
            .material
            .as_ref()
            .expect("Expected a material object in info");

        let mut rng = rand::thread_rng();
        let rnd_russian = rng.gen::<f64>();
        if depth < MAX_DEPTH || rnd_russian < self.continue_p {
            let specular_a = material.specular.unwrap_or([0., 0., 0.]);
            let diffuse_a = material.diffuse.unwrap_or([0., 0., 0.]);
            let specular = Color::from_column_slice(&[
                specular_a[0] as f64,
                specular_a[1] as f64,
                specular_a[2] as f64,
            ]);
            let diffuse = Color::from_column_slice(&[
                diffuse_a[0] as f64,
                diffuse_a[1] as f64,
                diffuse_a[2] as f64,
            ]);

            let s_p = if specular.y + diffuse.y != 0. {
                specular.y / (specular.y + diffuse.y)
            } else {
                0.0
            };
            let rnd = rng.gen::<f64>();

            let l_color = if rnd <= s_p || s_p >= (1. - f64::EPSILON) {
                // TODO: the s_p maybe is wrong as the image now is bad
                self.specular_reflection(intersection, &specular_a, scene, depth) / s_p
            } else {
                self.diffuse_reflection(intersection, &diffuse_a, scene, depth) / (1. - s_p)
            };
            color += if depth < MAX_DEPTH {
                l_color
            } else {
                l_color / self.continue_p
            };
        }

        if let Some(diffuse) = material.diffuse {
            if !diffuse.is_zero() {
                color += self.direct_lighting(intersection, material, scene);
            }
        }

        color
    }
}
