use std::error::Error;

use anyhow::anyhow;
use serde::Deserialize;

use crate::{camera::CameraArgs, image::Image, light::LightArgs, renderer::Renderer, scene::Scene};

#[derive(Debug, Deserialize)]
pub struct Configuration {
    model_file: String,
    samples_per_pixel: usize,
    lights: Vec<LightArgs>,
    camera: CameraArgs,
    #[serde(default = "default_output_file")]
    pub output_file: String,
}

fn default_output_file() -> String {
    "output.png".into()
}

pub struct RayTracer {
    renderer: Renderer,
}

impl RayTracer {
    pub fn with_configuration(configuration: Configuration) -> anyhow::Result<Self> {
        if !Image::valid_format(&configuration.output_file) {
            return Err(anyhow!("invalid extension of output file"));
        }

        let lights = configuration
            .lights
            .into_iter()
            .map(|light| light.into())
            .collect();
        Ok(RayTracer {
            renderer: Renderer::new(
                Scene::with_camera_args(&configuration.model_file, configuration.camera, lights),
                configuration.samples_per_pixel,
            ),
        })
    }

    pub fn new(
        obj_path: &str,
        camera_path: &str,
        samples_per_pixel: usize,
    ) -> Result<RayTracer, Box<dyn Error>> {
        Ok(RayTracer {
            renderer: Renderer::new(Scene::new(obj_path, camera_path)?, samples_per_pixel),
        })
    }

    pub fn render(&self, output_file: &str) {
        let image = self.renderer.render().unwrap();
        image.save(output_file).unwrap();
    }
}
