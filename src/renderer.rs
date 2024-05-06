use std::sync::{Arc, Mutex};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use nalgebra::Vector2;
use rand::Rng;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

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

        let image = Mutex::new(Image::new(width, height)?);
        (0..height).into_par_iter().for_each(|y| {
            (0..width).into_par_iter().for_each(|x| {
                samples_pb.reset();
                let color = (0..self.samples_per_pixel).into_par_iter().fold(Color::default, |a, _| {
                    let mut rng = rand::thread_rng();
                    let jitter = Vector2::new(rng.gen::<f64>(), rng.gen::<f64>());
                    let intersection = self.scene.cast_ray(x, y, jitter);
                    let color = shader.shade(&intersection, &self.scene, None);
                    samples_pb.inc(1);
                    a + color
                }).reduce(Color::default, |a, b| a + b);
                image
                    .lock()
                    .unwrap()
                    .set_pixel(x, y, color / self.samples_per_pixel as f64)
                    .expect("Error setting the pixel color");
                pixels_pb.inc(1);
            });
        });
        Ok(image.into_inner().unwrap())
    }
}
