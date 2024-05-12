use std::error::Error;

use nalgebra::Vector2;
use tobj::{Material, GPU_LOAD_OPTIONS};

use crate::{
    camera::{Camera, CameraArgs},
    light::{area_light::AreaLight, light_sampler::UniformLightSampler, Light},
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
    light_sampler: UniformLightSampler,
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

    pub fn light_sampler(&self) -> &UniformLightSampler {
        &self.light_sampler
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
        let mut scene = Scene::default();
        let (models, materials) = tobj::load_obj(obj_path, &GPU_LOAD_OPTIONS)?;

        scene.camera = camera;
        scene.materials = materials?;

        println!("# of models: {}", models.len());
        println!("# of materials: {}", scene.materials.len());

        scene.objects = models.into_iter().map(Mesh::from).collect();

        scene.lights = lights.clone();
        scene.light_sampler = UniformLightSampler::new(lights);

        Ok(scene)
    }

    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        let geometric_lights: Vec<AreaLight> = self
            .lights
            .iter()
            .filter_map(|light| {
                if let Light::Area(light) = light {
                    Some(light.clone())
                } else {
                    None
                }
            })
            .collect();

        let intersection = get_min_intersection(ray, &self.objects);

        let light_intersection = get_min_intersection(ray, &geometric_lights);

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

    pub fn cast_ray(&self, x: usize, y: usize, jitter: Vector2<f64>) -> Option<Intersection> {
        let ray = self.camera.get_ray(x, y, jitter);
        self.trace(&ray)
    }
}
