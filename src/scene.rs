use nalgebra::{Point3, Vector3};
use tobj::{Material, GPU_LOAD_OPTIONS, OFFLINE_RENDERING_LOAD_OPTIONS};

use crate::object::mesh::Mesh;

#[derive(Debug, Default)]
pub struct Scene {
    materials: Vec<Material>,
    objects: Vec<Mesh>,
}

impl Scene {
    pub fn new(path: &str) -> Result<Self, tobj::LoadError> {
        Self::load_obj(path)
    }

    fn load_obj(obj_path: &str) -> Result<Self, tobj::LoadError> {
        let mut scene = Scene::default();
        let (models, materials) = tobj::load_obj(obj_path, &GPU_LOAD_OPTIONS)?;

        scene.materials = materials?;

        println!("# of models: {}", models.len());
        println!("# of materials: {}", scene.materials.len());

        scene.objects = models.into_iter().map(|model| Mesh::from(model)).collect();

        dbg!(&scene.objects);

        todo!("Implement loading of the scene");
    }
}
