use std::error::Error;

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
    camera: Camera,
}

impl Scene {
    pub fn new(obj_path: &str, camera_path: &str) -> Result<Self, Box<dyn Error>> {
        Self::load_obj(obj_path, camera_path)
    }

    fn load_obj(obj_path: &str, camera_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut scene = Scene::default();
        let (models, materials) = tobj::load_obj(obj_path, &GPU_LOAD_OPTIONS)?;

        scene.camera = Camera::load(camera_path)?;
        scene.materials = materials?;

        println!("# of models: {}", models.len());
        println!("# of materials: {}", scene.materials.len());

        scene.objects = models.into_iter().map(|model| Mesh::from(model)).collect();

        dbg!(&scene.objects);

        todo!("Implement loading of the scene");
    }

    fn render(&self) -> Image {
        let image = Image::new(self.camera.width(), self.camera.height());

        for h in 0..self.camera.height() {
            for w in 0..self.camera.width() {
                todo!("Implement rendering of the scene")
            }
        }
        image
    }

    fn cast_ray(&self, ray: &Ray) -> Option<Intersection> {
        let intersection = get_min_intersection(ray, &self.objects);

        todo!("Implement the actualization of the intersection");

        intersection
    }
}
