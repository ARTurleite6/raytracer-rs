use crate::{object::intersection::Intersection, scene::Scene};

pub struct LightSampleContext<'a> {
    pub intersection: &'a Intersection,
    pub scene: &'a Scene,
}

impl<'a> LightSampleContext<'a> {
    pub fn new(intersection: &'a Intersection, scene: &'a Scene) -> Self {
        Self {
            intersection,
            scene,
        }
    }
}
