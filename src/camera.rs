use nalgebra::Vector2;
use serde::Deserialize;

use crate::{
    helpers::{Mat3, Vec3},
    object::ray::Ray,
};

#[derive(Deserialize, Debug)]
pub struct CameraArgs {
    width: usize,
    height: usize,
    angle_x: f64,
    angle_y: f64,
    position: Vec3,
    up: Vec3,
    look_at: Vec3,
}

impl From<CameraArgs> for Camera {
    fn from(args: CameraArgs) -> Self {
        let position = Vec3::new(args.position.x, args.position.y, args.position.z);
        let up = Vec3::new(args.up.x, args.up.y, args.up.z);
        let look_at = Vec3::new(args.look_at.x, args.look_at.y, args.look_at.z);
        Self::new(
            args.width,
            args.height,
            args.angle_x,
            args.angle_y,
            position,
            up,
            look_at,
        )
    }
}

#[derive(Debug, Default)]
pub struct Camera {
    look_at: Vec3,
    position: Vec3,
    width: usize,
    height: usize,
    angle_w: f64,
    angle_h: f64,
    up: Vec3,
    forward: Vec3,
    right: Vec3,
    camera_to_world: Mat3,
}

impl Camera {
    pub fn new(
        width: usize,
        height: usize,
        angle_x: f64,
        angle_y: f64,
        position: Vec3,
        up: Vec3,
        look_at: Vec3,
    ) -> Self {
        let forward = (look_at - position).normalize();
        let angle_w = (angle_x / 2.0).tan();
        let angle_h = (angle_y / 2.0).tan();
        let right = forward.cross(&up).normalize();
        let camera_to_world = Mat3::from_columns(&[right, up, forward]);

        Self {
            look_at,
            position,
            width,
            height,
            angle_w,
            angle_h,
            up,
            forward,
            right,
            camera_to_world,
        }
    }

    pub fn get_ray(&self, x: usize, y: usize, jitter: Vector2<f64>) -> Ray {
        let xf = x as f64;
        let yf = y as f64;
        let xs = (2.0 * (xf + jitter.x) / self.width as f64) - 1.0;
        let ys = (2.0 * ((self.height as f64 - yf - 1.0) + jitter.y) / self.height as f64) - 1.0;

        let xc = xs * self.angle_w;
        let yc = ys * self.angle_h;

        Ray::new_with_coords(
            self.position,
            self.camera_to_world * Vec3::new(xc, yc, 1.0).normalize(),
            x,
            y,
        )
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn load(path: &str) -> std::io::Result<Camera> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        Ok(
            serde_json::from_reader::<std::io::BufReader<std::fs::File>, CameraArgs>(reader)?
                .into(),
        )
    }
}
