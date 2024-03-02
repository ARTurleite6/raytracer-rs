use tobj::Material;

use crate::helpers::Vec3;

use super::ray::Ray;

#[derive(Debug)]
pub struct Intersection {
    point: Vec3,
    geometry_normal: Vec3,
    shading_normal: Vec3,
    w_outgoing: Vec3,
    depth: f32,
    brdf: Material,
    face_id: usize,
}

impl Intersection {
    pub fn get_depth(&self) -> f32 {
        self.depth
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Vec3>;
}


pub fn get_depth(point: &Vec3, ray: &Ray) -> f32 {
   (point - ray.get_origin()).norm()
}