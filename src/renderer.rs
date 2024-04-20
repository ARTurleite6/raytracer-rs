use nalgebra::Vector2;
use rand::Rng;

use crate::{
    helpers::{Color, Vec3},
    image::Image,
    scene::Scene,
    shader::{distributed_shader::DistributedShader, Shader},
};

#[derive(Debug)]
pub struct Renderer {
    scene: Scene,
    samples_per_pixel: usize,
}

impl Renderer {
    pub fn new(scene: Scene, samples_per_pixel: usize) -> Self {
        Self {
            scene,
            samples_per_pixel,
        }
    }

    pub fn render(&self) -> Result<Image, Box<dyn std::error::Error>> {
        let width = self.scene.width();
        let height = self.scene.height();
        let mut image = Image::new(width, height)?;
        // let shader = AmbientShader::new(Vec3::new(0.05, 0.05, 0.55));
        let shader = DistributedShader::new(Vec3::new(0.05, 0.05, 0.55));

        for y in 0..height {
            for x in 0..width {
                let mut color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let mut rng = rand::thread_rng();
                    let jitter = Vector2::new(rng.gen::<f64>(), rng.gen::<f64>());
                    // let ray = self.camera.get_ray(w as f64, h as f64);
                    let intersection = self.scene.cast_ray(x, y, jitter);
                    color += shader.shade(&intersection, &self.scene, None);
                }
                image.set_pixel(x, y, color / self.samples_per_pixel as f64)?;
            }
        }
        Ok(image)
    }
}
