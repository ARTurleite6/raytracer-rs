use crate::helpers::Vec3;

pub trait ILight {
    fn l(&self) -> Vec3;
}

#[derive(Debug, Clone)]
pub struct AmbientLight {
    color: Vec3,
}

impl AmbientLight {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl ILight for AmbientLight {
    fn l(&self) -> Vec3 {
        self.color
    }
}

#[derive(Debug, Clone)]
pub enum Light {
    Ambient(AmbientLight),
}

impl Light {
    pub fn l(&self) -> Vec3 {
        match self {
            Self::Ambient(ambient_light) => ambient_light.l(),
        }
    }
}
