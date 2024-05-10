use nalgebra::Vector3;
use serde::Deserialize;

use crate::helpers::{Rotateable, Vec3};

use super::{
    bounding_box::BoundingBox,
    intersection::{Intersectable, Intersection},
    ray::Ray,
};

const EPSILON: f64 = 0.0001;

#[derive(Debug, Default)]
pub struct FaceBuilder {
    face_id: Option<usize>,
    vertex: [Vec3; 3],
    normal: Option<Vec3>,
}

impl FaceBuilder {
    pub fn new(vertex: [Vec3; 3]) -> Self {
        Self {
            vertex,
            ..Default::default()
        }
    }

    pub fn face_id(mut self, face_id: usize) -> Self {
        self.face_id = Some(face_id);
        self
    }

    pub fn build(self) -> Face {
        self.into()
    }
}

impl From<FaceBuilder> for Face {
    fn from(value: FaceBuilder) -> Self {
        Self::new(value.vertex, value.normal)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Face {
    vertex: [Vec3; 3],
    normal: Vec3,
    bounding_box: BoundingBox,
    area: f64,
}

impl Face {
    pub fn new(vertex: [Vec3; 3], normal: Option<Vec3>) -> Self {
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
        let normal = normal.unwrap_or(edge_1.cross(&edge_2).normalize());

        let edge_3 = vertex[2] - vertex[1];
        let a = edge_1.norm();
        let b = edge_2.norm();
        let c = edge_3.norm();

        let half_perimeter = (a + b + c) / 2.0;
        let area =
            (half_perimeter * (half_perimeter - a) * (half_perimeter - b) * (half_perimeter - c))
                .sqrt();

        Self {
            vertex,
            normal,
            bounding_box,
            area,
        }
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn vertices(&self) -> [Vec3; 3] {
        self.vertex
    }

    pub fn get_bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }

    pub fn area(&self) -> f64 {
        self.area
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
        if !(0.0..=1.0).contains(&u) {
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
            let wo = -1.0 * ray.direction();

            let normal = self.normal.face_forward(wo);
            Some(Intersection::new(
                intersection_point,
                normal,
                normal,
                wo,
                t,
                None,
            ))
        } else {
            None
        }
    }
}
