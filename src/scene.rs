use std::error::Error;

use nalgebra::Vector2;
use tobj::{Material, GPU_LOAD_OPTIONS};

use crate::{
    camera::{Camera, CameraArgs},
    light::{
        light_sampler::{power_sampler::PowerLightSampler, LightSampler},
        Light,
    },
    object::{
        intersection::{get_min_intersection, Intersectable, Intersection, MaterialInformation},
        mesh::Mesh,
        ray::Ray,
    },
};

pub struct Scene {
    materials: Vec<Material>,
    objects: Vec<Mesh>,
    lights: Vec<Light>,
    camera: Camera,
}

impl Scene {
    pub fn with_camera_args(obj_path: &str, camera_args: CameraArgs, lights: Vec<Light>) -> Self {
        let camera = camera_args.into();
        Self::load_obj(obj_path, camera, lights).expect("Error loading model config")
    }

    pub fn new(obj_path: &str, camera_path: &str) -> Result<Self, Box<dyn Error>> {
        let camera = Camera::load(camera_path)?;
        Self::load_obj(obj_path, camera, Vec::default())
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

    fn load_obj(
        obj_path: &str,
        camera: Camera,
        lights: Vec<Light>,
    ) -> Result<Self, Box<dyn Error>> {
        let (models, materials) = tobj::load_obj(obj_path, &GPU_LOAD_OPTIONS)?;

        let objects = models.into_iter().map(Mesh::from).collect();

        Ok(Self {
            lights,
            objects,
            camera,
            materials: materials?,
        })
    }

    pub fn create_light_sampler(&self) -> PowerLightSampler {
        PowerLightSampler::new(self.lights.iter())
    }

    pub fn trace<L: LightSampler>(&self, ray: &Ray, light_sampler: &L) -> Option<Intersection> {
        let geometric_lights = light_sampler.geometric_lights();

        let intersection = get_min_intersection(ray, self.objects.iter());

        let light_intersection = get_min_intersection(ray, geometric_lights);

        let mut min_intersection = [intersection, light_intersection]
            .into_iter()
            .flatten()
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

    pub fn cast_ray<L: LightSampler>(
        &self,
        x: usize,
        y: usize,
        jitter: &Vector2<f64>,
        light_sampler: &L,
    ) -> Option<Intersection> {
        let ray = self.camera.get_ray(x, y, jitter);
        self.trace(&ray, light_sampler)
    }
}
