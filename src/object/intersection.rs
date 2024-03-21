use tobj::Material;

use crate::helpers::Vec3;

use super::ray::Ray;

#[derive(Debug, Clone)]
pub struct MaterialInformation {
    pub material: Option<Material>,
    pub material_id: usize,
}

#[derive(Debug)]
pub struct Intersection {
    point: Vec3,
    geometry_normal: Vec3,
    shading_normal: Vec3,
    w_outgoing: Vec3,
    depth: f32,
    pub brdf: Option<MaterialInformation>,
    face_id: usize,
}

impl Intersection {
    pub fn new(
        point: Vec3,
        geometry_normal: Vec3,
        shading_normal: Vec3,
        w_outgoing: Vec3,
        depth: f32,
        face_id: usize,
    ) -> Self {
        Self {
            point,
            geometry_normal,
            shading_normal,
            w_outgoing,
            depth,
            brdf: None,
            face_id,
        }
    }

    pub fn get_depth(&self) -> f32 {
        self.depth
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

pub fn get_min_intersection<T: Intersectable>(ray: &Ray, objects: &[T]) -> Option<Intersection> {
    let mut min_intersection: Option<Intersection> = None;

    for object in objects.iter() {
        if let Some(intersection) = object.intersect(ray) {
            if let Some(curr_intersection) = &min_intersection {
                if intersection.get_depth() < curr_intersection.get_depth() {
                    min_intersection = Some(intersection);
                }
            } else {
                min_intersection = Some(intersection);
            }
        }
    }
    min_intersection
}

pub fn get_depth(point: &Vec3, ray: &Ray) -> f32 {
    (point - ray.get_origin()).norm()
}
