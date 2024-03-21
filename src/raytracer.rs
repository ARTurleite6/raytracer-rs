use std::error::Error;

use crate::{renderer::Renderer, scene::Scene};

#[derive(Debug)]
pub struct RayTracer {
    renderer: Renderer,
}

impl RayTracer {
    pub fn new(obj_path: &str, camera_path: &str) -> Result<RayTracer, Box<dyn Error>> {
        Ok(RayTracer {
            renderer: Renderer::new(Scene::new(obj_path, camera_path)?),
        })
    }

    pub fn render(&self) {
        let image = self.renderer.render().unwrap();
        image.save("output.png").unwrap();
    }
}
