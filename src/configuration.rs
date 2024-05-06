use serde::Deserialize;

use crate::{camera::CameraArgs, light::Light};

#[derive(Debug, Deserialize)]
pub struct Configuration {
    model_file: String,
    samples_per_pixel: usize,
    camera: CameraArgs,
    lights: Vec<Light>,
}
