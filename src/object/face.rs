use nalgebra::{Vector2, Vector3};

use crate::helpers::Vec3;

use super::{
    bounding_box::BoundingBox,
    intersection::{Intersectable, Intersection},
    ray::Ray,
};

const EPSILON: f64 = 0.0001;

#[derive(Debug)]
pub struct Face {
    face_id: usize,
    vertex: [Vec3; 3],
    normal: Vec3,
    // normal_coordinates: Option<[Vector3<f64>; 3]>,
    // texture_coordinates: Option<[Vector2<f64>; 3]>,
    bounding_box: BoundingBox,
}

impl Face {
    pub fn new(
        face_id: usize,
        vertex: [Vec3; 3],
        normal_coordinates: Option<[Vec3; 3]>,
        texture_coordinates: Option<[Vec3; 3]>,
    ) -> Self {
        let bounding_box = BoundingBox::new(
            vertex.iter().fold(vertex[0], |acc, new_vertex| {
                Vector3::new(
                    acc.x.min(new_vertex.x),
                    acc.y.min(new_vertex.y),
                    acc.z.min(new_vertex.z),
                )
            }),
            vertex.iter().fold(vertex[0], |acc, new_vertex| {
                Vector3::new(
                    acc.x.max(new_vertex.x),
                    acc.y.max(new_vertex.y),
                    acc.z.max(new_vertex.z),
                )
            }),
        );

        let edge_1 = vertex[1] - vertex[0];
        let edge_2 = vertex[2] - vertex[0];
        let normal = edge_1.cross(&edge_2).normalize();

        Self {
            face_id,
            vertex,
            normal,
            bounding_box,
        }
    }

    pub fn get_bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}

impl Intersectable for Face {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if !self.bounding_box.intersect(ray) {
            return None;
        }

        let edge_1 = self.vertex[1] - self.vertex[0];
        let edge_2 = self.vertex[2] - self.vertex[0];
        let ray_cross_e2 = ray.direction().cross(&edge_2);
        let det = edge_1.dot(&ray_cross_e2);

        if det > -EPSILON && det < EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let s = ray.origin() - self.vertex[0];
        let u = inv_det * s.dot(&ray_cross_e2);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let s_cross_e1 = s.cross(&edge_1);
        let v = inv_det * ray.direction().dot(&s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = inv_det * edge_2.dot(&s_cross_e1);

        if t > EPSILON {
            let intersection_point = ray.origin() + ray.direction() * t;

            Some(Intersection::new(
                intersection_point,
                self.normal,
                self.normal,
                -1.0 * ray.direction(),
                t,
                self.face_id,
            ))
        } else {
            None
        }
    }
}
