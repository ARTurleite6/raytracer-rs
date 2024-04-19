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
    depth: f64,
    pub brdf: Option<MaterialInformation>,
    face_id: usize,
}

impl Intersection {
    pub fn new(
        point: Vec3,
        geometry_normal: Vec3,
        shading_normal: Vec3,
        w_outgoing: Vec3,
        depth: f64,
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

    pub fn face_id(&self) -> usize {
        self.face_id
    }

    pub fn depth(&self) -> f64 {
        self.depth
    }

    pub fn point(&self) -> Vec3 {
        self.point
    }

    pub fn w_outgoing(&self) -> Vec3 {
        self.w_outgoing
    }

    pub fn shading_normal(&self) -> Vec3 {
        self.shading_normal
    }

    pub fn geometric_normal(&self) -> Vec3 {
        self.geometry_normal
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
                if intersection.depth() < curr_intersection.depth() {
                    min_intersection = Some(intersection);
                }
            } else {
                min_intersection = Some(intersection);
            }
        }
    }
    min_intersection
}

pub fn get_depth(point: &Vec3, ray: &Ray) -> f64 {
    (point - ray.origin()).norm()
}
