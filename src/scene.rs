use std::error::Error;

use nalgebra::Vector2;
use tobj::{Material, GPU_LOAD_OPTIONS};

use crate::{
    camera::Camera,
    helpers::{Color, Vec3},
    light::{ambient_light::AmbientLight, area_light::AreaLight, point_light::PointLight, Light},
    object::{
        intersection::{get_min_intersection, Intersectable, Intersection, MaterialInformation},
        mesh::Mesh,
        ray::Ray,
    },
};

#[derive(Debug, Default)]
pub struct Scene {
    materials: Vec<Material>,
    objects: Vec<Mesh>,
    lights: Vec<Light>,
    camera: Camera,
}

impl Scene {
    pub fn new(obj_path: &str, camera_path: &str) -> Result<Self, Box<dyn Error>> {
        Self::load_obj(obj_path, camera_path)
    }

    pub fn width(&self) -> usize {
        self.camera.width()
    }

    pub fn height(&self) -> usize {
        self.camera.height()
    }

    pub fn lights(&self) -> &[Light] {
        &self.lights
    }

    pub fn visibility(&self, ray: &Ray, max_l: f64) -> bool {
        !self.objects.iter().any(|object| {
            object
                .intersect(ray)
                .map_or(false, |intersection| intersection.depth() < max_l)
        })
    }

    fn load_obj(obj_path: &str, camera_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut scene = Scene::default();
        let (models, materials) = tobj::load_obj(obj_path, &GPU_LOAD_OPTIONS)?;

        scene.camera = Camera::load(camera_path)?;
        scene.materials = materials?;

        println!("# of models: {}", models.len());
        println!("# of materials: {}", scene.materials.len());

        scene.objects = models.into_iter().map(Mesh::from).collect();

        scene.lights = vec![
            Light::Ambient(AmbientLight::new(Vec3::new(0.05, 0.05, 0.05))),
            Light::AreaLight(AreaLight::new(
                [
                    Vec3::new(200.0, 508.0, 200.0),
                    Vec3::new(316.0, 508.0, 200.0),
                    Vec3::new(316.0, 508.0, 316.0),
                ],
                Vec3::new(0.0, -1.0, 0.0),
                Color::new(1.5, 1.5, 1.5),
            )),
            Light::AreaLight(AreaLight::new(
                [
                    Vec3::new(200.0, 508.0, 200.0),
                    Vec3::new(316.0, 508.0, 316.0),
                    Vec3::new(200.0, 508.0, 316.0),
                ],
                Vec3::new(0.0, -1.0, 0.0),
                Color::new(1.5, 1.5, 1.5),
            )),
        ];

        Ok(scene)
    }

    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        let geometric_lights: Vec<AreaLight> = self
            .lights
            .iter()
            .filter_map(|light| {
                if let Light::AreaLight(light) = light {
                    Some(light.clone())
                } else {
                    None
                }
            })
            .collect();

        let intersection = get_min_intersection(&ray, &self.objects);

        let light_intersection = get_min_intersection(&ray, &geometric_lights);

        let mut min_intersection = [intersection, light_intersection]
            .into_iter()
            .filter_map(|intersection| intersection)
            .min_by(|a, b| a.depth().total_cmp(&b.depth()))?;

        if !min_intersection.is_light() {
            if let Some(MaterialInformation {
                material_id,
                material: _,
            }) = &mut min_intersection.brdf
            {
                min_intersection.brdf = Some(MaterialInformation {
                    material_id: *material_id,
                    material: Some(self.materials[*material_id].clone()),
                });
            }
        }
        Some(min_intersection)
    }

    pub fn cast_ray(&self, x: usize, y: usize, jitter: Vector2<f64>) -> Option<Intersection> {
        let ray = self.camera.get_ray(x, y, jitter);
        self.trace(&ray)
    }
}
