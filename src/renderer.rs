use nalgebra::{Vector2, Vector3};
use rand::Rng;

use crate::{image::Image, scene::Scene};

#[derive(Debug)]
pub struct Renderer {
    scene: Scene,
}

impl Renderer {
    pub fn new(scene: Scene) -> Self {
        Self { scene }
    }

    pub fn render(&self) -> Result<Image, Box<dyn std::error::Error>> {
        let width = self.scene.width();
        let height = self.scene.height();
        let mut image = Image::new(width, height)?;
        let color = Vector3::<f32>::new(0.5, 0.5, 0.5);

        for y in 0..height {
            for x in 0..width {
                let mut rng = rand::thread_rng();
                let jitter = Vector2::new(rng.gen::<f32>(), rng.gen::<f32>());
                // let ray = self.camera.get_ray(w as f64, h as f64);
                let intersection = self.scene.cast_ray(x, y, jitter);
                image.set_pixel(x, y, color)?;
            }
        }
        Ok(image)
    }
}
