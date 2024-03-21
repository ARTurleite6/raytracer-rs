use std::error::Error;

use nalgebra::{Vector2, Vector3};
use tobj::{Material, GPU_LOAD_OPTIONS};

use crate::{
    camera::Camera,
    image::Image,
    object::{
        intersection::{get_min_intersection, Intersection},
        mesh::Mesh,
        ray::Ray,
    },
};

#[derive(Debug, Default)]
pub struct Scene {
    materials: Vec<Material>,
    objects: Vec<Mesh>,
    // lights: Vec<Light>,
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

    fn load_obj(obj_path: &str, camera_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut scene = Scene::default();
        let (models, materials) = tobj::load_obj(obj_path, &GPU_LOAD_OPTIONS)?;

        scene.camera = Camera::load(camera_path)?;
        scene.materials = materials?;

        println!("# of models: {}", models.len());
        println!("# of materials: {}", scene.materials.len());

        scene.objects = models.into_iter().map(Mesh::from).collect();

        dbg!(&scene.objects);
        Ok(scene)
    }

    pub fn cast_ray(&self, x: usize, y: usize, jitter: Vector2<f32>) -> Option<Intersection> {
        let ray = self.camera.get_ray(x, y, jitter);
        let intersection = get_min_intersection(&ray, &self.objects)?;
        Some(intersection)
    }
}
