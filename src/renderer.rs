use nalgebra::Vector2;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    helpers::{Color, Vec3},
    image::Image,
    scene::Scene,
    shader::{
        better_path_tracer_shader::PathTracer, path_tracer_shader::PathTracerShader, BetterShader,
        Shader,
    },
};

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
        let _shader = PathTracerShader::new(Vec3::new(0.05, 0.05, 0.55));
        let shader = PathTracer::new(Vec3::new(0.05, 0.05, 0.55));

        let pixels_color: Vec<Color> = (0..height)
            .into_par_iter()
            .flat_map(|y| (0..width).into_par_iter().map(move |x| (y, x)))
            .map(|(y, x)| {
                let color =
                    (0..self.samples_per_pixel)
                        .into_iter()
                        .fold(Color::default(), |a, _| {
                            let mut rng = fastrand::Rng::new();
                            let jitter = Vector2::new(rng.f64(), rng.f64());
                            let intersection = self.scene.cast_ray(x, y, &jitter);
                            let color = shader.shade(&intersection, &self.scene, None, &mut rng);
                            // let color = shader.shade(&intersection, &self.scene, None);
                            a + color
                        });
                color / self.samples_per_pixel as f64
            })
            .collect();

        Ok(Image::new(width, height, pixels_color))
    }
}
