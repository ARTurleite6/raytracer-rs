use nalgebra::Vector2;
use rand::Rng;

use crate::{
    helpers::Vec3,
    image::Image,
    scene::Scene,
    shader::{ambient_shader::AmbientShader, Shader},
};

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
        let shader = AmbientShader::new(Vec3::new(0.05, 0.05, 0.55));
        let lights = self.scene.lights();

        for y in 0..height {
            for x in 0..width {
                let mut rng = rand::thread_rng();
                let jitter = Vector2::new(rng.gen::<f32>(), rng.gen::<f32>());
                // let ray = self.camera.get_ray(w as f64, h as f64);
                let intersection = self.scene.cast_ray(x, y, jitter);
                let color = shader.shade(&intersection, lights);
                image.set_pixel(x, y, color)?;
            }
        }
        Ok(image)
    }
}
