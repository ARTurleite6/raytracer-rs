use std::error::Error;

use bvh::{bounding_hierarchy::BoundingHierarchy, bvh::Bvh};
use nalgebra::{Point3, Vector2};
use tobj::{Material, GPU_LOAD_OPTIONS};

use crate::{
    camera::{Camera, CameraArgs},
    helpers::Vec3,
    light::{
        area_light::AreaLight,
        light_sampler::{power_sampler::PowerLightSampler, LightSampler},
        Light,
    },
    object::{
        face::Face,
        intersection::{get_min_intersection, Intersectable, Intersection, MaterialInformation},
        mesh::Mesh,
        ray::Ray,
    },
};

pub struct Scene {
    faces: Vec<Face>,
    materials: Vec<Material>,
    objects: Vec<Mesh>,
    lights: Vec<Light>,
    light_sampler: Box<dyn LightSampler + Send + Sync>,
    geometric_lights: Vec<AreaLight>,
    bvh: Bvh<f64, 3>,
    camera: Camera,
}

impl Scene {
    pub fn with_camera_args(obj_path: &str, camera_args: CameraArgs, lights: Vec<Light>) -> Self {
        let camera = camera_args.into();
        Self::load_obj(obj_path, camera, lights).expect("Error loading model config")
    }

    pub fn new(obj_path: &str, camera_path: &str) -> Result<Self, Box<dyn Error>> {
        let camera = Camera::load(camera_path)?;
        Self::load_obj(obj_path, camera, Vec::default())
    }

    pub fn width(&self) -> usize {
        self.camera.width()
    }

    pub fn height(&self) -> usize {
        self.camera.height()
    }

    pub fn light_sampler(&self) -> &dyn LightSampler {
        self.light_sampler.as_ref()
    }

    pub fn lights(&self) -> &[Light] {
        &self.lights
    }

    pub fn visibility(&self, ray: &Ray, max_l: f64) -> bool {
        !self.objects.iter().any(|object| {
            object
                .intersect(ray)
                .map_or(false, |intersection| intersection.depth() < max_l)
        })
    }

    fn load_obj(
        obj_path: &str,
        camera: Camera,
        lights: Vec<Light>,
    ) -> Result<Self, Box<dyn Error>> {
        let (models, materials) = tobj::load_obj(obj_path, &GPU_LOAD_OPTIONS)?;

        let objects: Vec<Mesh> = models.into_iter().map(Mesh::from).collect();

        let light_sampler = Box::new(PowerLightSampler::new(lights.clone()));
        let geometric_lights = light_sampler.geometric_lights();
        let mut faces: Vec<Face> = objects.iter().flat_map(|obj| obj.faces().clone()).collect();
        dbg!(&faces);
        let bvh = bvh::bvh::Bvh::build_par(&mut faces);
        dbg!(&faces);
        bvh.pretty_print();

        Ok(Self {
            bvh,
            faces,
            lights,
            objects,
            camera,
            materials: materials?,
            geometric_lights,
            light_sampler,
        })
    }

    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        let geometric_lights = &self.geometric_lights;
        //let intersection = get_min_intersection(ray, &self.faces);
        // dbg!(intersection);
        let hit = self.bvh.traverse(
            &bvh::ray::Ray::new(Point3::from(*ray.origin()), *ray.direction()),
            &self.faces,
        );

        if hit.len() > 0 {
            dbg!("ola");
        }

        let intersection = hit
            .into_iter()
            .filter_map(|face| face.intersect(ray))
            .min_by(|intersection, other| intersection.depth().total_cmp(&other.depth()));

        let light_intersection = get_min_intersection(ray, &geometric_lights);

        let mut min_intersection = [intersection, light_intersection]
            .into_iter()
            .flatten()
            .min_by(|a, b| a.depth().total_cmp(&b.depth()))?;

        if !min_intersection.is_light() {
            if let Some(MaterialInformation {
                material_id,
                material: _,
            }) = &mut min_intersection.brdf
            {
                min_intersection.brdf = Some(MaterialInformation {
                    material_id: *material_id,
                    material: Some(self.materials[*material_id].clone()),
                });
            }
        }
        Some(min_intersection)
    }

    pub fn cast_ray(&self, x: usize, y: usize, jitter: &Vector2<f64>) -> Option<Intersection> {
        let ray = self.camera.get_ray(x, y, jitter);
        self.trace(&ray)
    }
}
