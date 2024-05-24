use std::f64::consts::PI;

use fastrand::Rng;
use tobj::Material;

use crate::{
    helpers::{Color, CoordinateSystemProvider, Rotateable, Vec3, Zeroable},
    light::{
        light_sample_context::LightSampleContext,
        light_sampler::{self, LightSampler, SampleLight},
        Light, SampleLightResult,
    },
    object::{intersection::Intersection, ray::Ray},
    scene::Scene,
};

use super::BetterShader;

type Brdf = [f32; 3];

const MAX_DEPTH: u32 = 2;

pub struct PathTracer {
    background_color: Color,
    continue_p: f64,
}

impl PathTracer {
    pub fn new(background_color: Color) -> Self {
        Self {
            background_color,
            continue_p: 0.5,
        }
    }

    pub fn direct_lighting<L: LightSampler>(
        &self,
        intersection: &Intersection,
        material: &Material,
        scene: &Scene,
        light_sampler: &L,
        rng: &mut Rng,
    ) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);

        if let Some(ambient) = material.ambient {
            if !ambient.is_zero() {
                let ambient_color = light_sampler.sample_ambient_lights(ambient);
                color += ambient_color;
            }
        }

        if let Some(SampleLight {
            light: light_sampled,
            power,
            sample_result,
        }) = light_sampler.sample(LightSampleContext::new(intersection, &scene), rng)
        {
            match light_sampled {
                Light::Area(_) => {
                    if let Some(diffuse) = material.diffuse {
                        if !diffuse.is_zero() {
                            let SampleLightResult {
                                color: light_color,
                                pdf,
                                cos,
                                distance: light_distance,
                                light_dir,
                                ..
                            } = sample_result;
                            let i_point = intersection.point();
                            let light_dir = light_dir.unwrap();
                            let light_distance = light_distance.unwrap();
                            let cos_l = cos.unwrap();

                            let mut shadow = Ray::new(&i_point, &light_dir);
                            shadow.adjust_origin(intersection.geometric_normal());

                            if scene.visibility(&shadow, light_distance - 0.0001) {
                                let diffuse =
                                    [diffuse[0] as f64, diffuse[1] as f64, diffuse[2] as f64];

                                color += (Vec3::from(diffuse).component_mul(&light_color) * cos_l
                                    / power)
                                    / pdf.unwrap()
                            }
                        }
                    }
                }

                Light::Point(_) => {
                    if let Some(diffuse) = material.diffuse {
                        if !diffuse.is_zero() {
                            let SampleLightResult {
                                color: light_color,
                                light_dir,
                                distance: light_distance,
                                cos,
                                ..
                            } = sample_result;
                            let light_dir = light_dir.unwrap();
                            let light_distance = light_distance.unwrap();
                            let cos = cos.unwrap();

                            let mut shadow = Ray::new(&intersection.point(), &light_dir);
                            shadow.adjust_origin(intersection.geometric_normal());
                            if scene.visibility(&shadow, light_distance) {
                                let diffuse =
                                    [diffuse[0] as f64, diffuse[1] as f64, diffuse[2] as f64];
                                color +=
                                    (Vec3::from(diffuse).component_mul(&light_color) * cos) / power;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        color
    }

    fn specular_reflection<L: LightSampler>(
        &self,
        intersection: &Intersection,
        material: &Brdf,
        scene: &Scene,
        depth: u32,
        light_sampler: &L,
        rng: &mut Rng,
    ) -> Color {
        let gn = intersection.geometric_normal();
        let wo = intersection.w_outgoing();

        let cos = gn.dot(&wo);

        let r_dir = 2.0 * cos * gn - wo;
        let specular = Ray::new_with_adjusted_origin(&intersection.point(), &r_dir, &gn);

        let specular_intersection = scene.trace(&specular, light_sampler);
        let r_color = self.shade(
            &specular_intersection,
            scene,
            Some(depth + 1),
            light_sampler,
            rng,
        );

        Vec3::from_column_slice(&[material[0] as f64, material[1] as f64, material[2] as f64])
            .component_mul(&r_color)
    }

    fn diffuse_reflection<L: LightSampler>(
        &self,
        intersection: &Intersection,
        material: &Brdf,
        scene: &Scene,
        depth: u32,
        light_sampler: &L,
        rng: &mut Rng,
    ) -> Color {
        let randoms = [rng.f64(), rng.f64()];

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

        let diffuse = Ray::new_with_adjusted_origin(
            intersection.point(),
            &d_around.rotate(&rx, &ry, &gn),
            gn,
        );
        let d_intersection = scene.trace(&diffuse, light_sampler);
        if let Some(d_intersection) = d_intersection {
            if !d_intersection.is_light() {
                let r_color = self.shade(
                    &d_intersection.into(),
                    scene,
                    Some(depth + 1),
                    light_sampler,
                    rng,
                );

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
}

impl BetterShader for PathTracer {
    fn shade<L: LightSampler>(
        &self,
        intersection: &Option<Intersection>,
        scene: &Scene,
        depth: Option<u32>,
        light_sampler: &L,
        rng: &mut Rng,
    ) -> Color {
        let mut color = Color::default();
        let Some(intersection) = intersection else {
            return self.background_color;
        };

        let depth = depth.unwrap_or(0);
        if intersection.is_light() {
            return intersection.light_intensity.unwrap();
        }

        let Some(material) = intersection.brdf() else {
            return color;
        };

        let rnd_russian = rng.f64();

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
            let rnd_spec = rng.f64();

            let l_color = if rnd_spec <= s_p || s_p >= (1. - f64::EPSILON) {
                self.specular_reflection(
                    intersection,
                    &specular_a,
                    scene,
                    depth,
                    light_sampler,
                    rng,
                ) / s_p
            } else {
                self.diffuse_reflection(intersection, &diffuse_a, scene, depth, light_sampler, rng)
                    / (1. - s_p)
            };
            color += if depth < MAX_DEPTH {
                l_color
            } else {
                l_color / self.continue_p
            };
        }

        if let Some(diffuse) = material.diffuse {
            if !diffuse.is_zero() {
                color += self.direct_lighting(intersection, material, scene, light_sampler, rng);
            }
        }

        color
    }
}
