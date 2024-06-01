use bvh::{
    aabb::{Aabb, Bounded},
    bounding_hierarchy::BHShape,
};
use nalgebra::{Point3, Vector3};
use serde::Deserialize;

use crate::helpers::{Rotateable, Vec3};

use super::{
    bounding_box::BoundingBox,
    intersection::{Intersectable, Intersection, MaterialInformation},
    ray::Ray,
};

const EPSILON: f64 = 0.0001;

#[derive(Debug, Default)]
pub struct FaceBuilder {
    material_id: Option<usize>,
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

    pub fn material_id(mut self, material_id: usize) -> Self {
        self.material_id = Some(material_id);
        self
    }

    pub fn normal(mut self, normal: &Vec3) -> Self {
        self.normal = Some(*normal);
        self
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
        let mut face = Self::new(value.face_id.unwrap_or(0), value.vertex, value.normal);
        face.material_id = value.material_id;
        face
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Face {
    face_id: usize,
    material_id: Option<usize>,
    vertex: [Vec3; 3],
    normal: Vec3,
    bounding_box: BoundingBox,
    area: f64,
}

impl Face {
    pub fn new(face_id: usize, vertex: [Vec3; 3], normal: Option<Vec3>) -> Self {
        let bounding_box = BoundingBox::new(
            &vertex.iter().fold(vertex[0], |acc, new_vertex| {
                Vector3::new(
                    acc.x.min(new_vertex.x),
                    acc.y.min(new_vertex.y),
                    acc.z.min(new_vertex.z),
                )
            }),
            &vertex.iter().fold(vertex[0], |acc, new_vertex| {
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
            face_id,
            material_id: None,
            vertex,
            normal,
            bounding_box,
            area,
        }
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn vertices(&self) -> &[Vec3; 3] {
        &self.vertex
    }

    pub fn get_bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }

    pub fn area(&self) -> f64 {
        self.area
    }
}

impl Bounded<f64, 3> for Face {
    fn aabb(&self) -> bvh::aabb::Aabb<f64, 3> {
        let (min, max) = self.bounding_box.get_min_max();

        Aabb::with_bounds(Point3::from(*min), Point3::from(*max))
    }
}

impl BHShape<f64, 3> for Face {
    fn set_bh_node_index(&mut self, face_id: usize) {
        self.face_id = face_id
    }

    fn bh_node_index(&self) -> usize {
        self.face_id
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

            let normal = self.normal.face_forward(&wo);
            let mut intersection =
                Intersection::new(intersection_point, normal, normal, wo, t, None);
            if let Some(material_id) = self.material_id {
                intersection.brdf = Some(MaterialInformation {
                    material_id,
                    ..Default::default()
                });
            }
            Some(intersection)
        } else {
            None
        }
    }
}
