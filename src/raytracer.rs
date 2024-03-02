use crate::scene::Scene;

#[derive(Debug)]
pub struct RayTracer {
    scene: Scene,
}

impl RayTracer {
    pub fn new() -> RayTracer {
        RayTracer {
            // TODO: load camera
            scene: Scene::new("models/cornell_box.obj", "").unwrap(),
        }
    }

    pub fn render(&self) {
        println!("Rendering...");
    }
}
