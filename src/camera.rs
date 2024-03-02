use serde::Deserialize;

use crate::helpers::{Mat3, Vec3};

#[derive(Debug, Deserialize)]
struct MyVec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Default)]
pub struct Camera {
    look_at: Vec3,
    position: Vec3,
    width: u32,
    height: u32,
    angle_w: f64,
    angle_h: f64,
    up: Vec3,
    forward: Vec3,
    right: Vec3,
    camera_to_world: Mat3,
}

#[derive(Deserialize, Debug)]
pub struct CameraArgs {
    width: u32,
    height: u32,
    angle_x: f64,
    angle_y: f64,
    position: MyVec3,
    up: MyVec3,
    look_at: MyVec3,
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

impl Camera {
    pub fn new(
        width: u32,
        height: u32,
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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn load(path: &str) -> std::io::Result<Camera> {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        Ok(serde_json::from_reader::<std::io::BufReader<std::fs::File>, CameraArgs>(reader)
            ?
            .into())
    }
}
