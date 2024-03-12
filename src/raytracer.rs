use std::error::Error;

use crate::scene::Scene;

#[derive(Debug)]
pub struct RayTracer {
    scene: Scene,
}

impl RayTracer {
    pub fn new(obj_path: &str, camera_path: &str) -> Result<RayTracer, Box<dyn Error>> {
        Ok(RayTracer {
            // TODO: load camera
            scene: Scene::new(obj_path, camera_path)?,
        })
    }

    pub fn render(&self) {
        println!("Rendering...");
    }
}
