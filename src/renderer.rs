use std::sync::{Arc, Mutex};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use nalgebra::Vector2;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    helpers::{Color, Vec3},
    image::Image,
    scene::Scene,
    shader::{path_tracer_shader::PathTracerShader, Shader},
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
        // let shader = AmbientShader::new(Vec3::new(0.05, 0.05, 0.55));
        // let shader = DistributedShader::new(Vec3::new(0.05, 0.05, 0.55));
        let shader = PathTracerShader::new(Vec3::new(0.05, 0.05, 0.55));

        let multi_progress_bar = MultiProgress::new();
        let pixels_pb = multi_progress_bar.add(ProgressBar::new((width * height) as u64));
        let samples_pb = multi_progress_bar.add(ProgressBar::new(self.samples_per_pixel as u64));
        let samples_style = ProgressStyle::default_bar()
            .template("SAMPLES RAN IN CURRENT PIXEL: [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} (ETA {eta})").expect("Error creating style for samples");
        let pixels_style = ProgressStyle::default_bar()
            .template("PIXELS DRAWN: [{elapsed_precise}] {bar:40.green/red} {pos}/{len} ({percent}%) (ETA {eta})").expect("Error creating style for pixels");

        samples_pb.set_style(samples_style);
        pixels_pb.set_style(pixels_style);

        let image_data: Vec<Vec<Color>> = (0..height).into_iter().map(|y| {
            (0..width).into_iter().map(|x| {
                let mut color = Color::new(0.0, 0.0, 0.0);
                samples_pb.reset();
                for _ in 0..self.samples_per_pixel {
                    let mut rng = rand::thread_rng();
                    let jitter = Vector2::new(rng.gen::<f64>(), rng.gen::<f64>());
                    let intersection = self.scene.cast_ray(x, y, jitter);
                    color += shader.shade(&intersection, &self.scene, None);
                    samples_pb.inc(1);
                }
                // image.set_pixel(x, y, color / self.samples_per_pixel as f64)?;
                pixels_pb.inc(1);
                color / self.samples_per_pixel as f64
            }).collect()
        }).collect();
        Ok(Image::with_image_data(image_data))
    }
}
