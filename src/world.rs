use crate::scene::{Scene, HitRecord};
use crate::ray::Ray;

pub struct World {
    pub objects: Vec<Box<dyn Scene>>,
}

impl World {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Box<dyn Scene>) {
        self.objects.push(object);
    }
}

impl Scene for World {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let mut closest: Option<HitRecord> = None;

        for obj in &self.objects {
            if let Some(hit) = obj.hit(ray) {
                if closest.is_none() || hit.t < closest.as_ref().unwrap().t {
                    closest = Some(hit);
                }
            }
        }

        closest
    }

    fn background_colour(&self, ray: &Ray) -> crate::vec3::Vec3 {
        // Default background color (e.g., black)
        crate::vec3::Vec3::new(0.0, 0.0, 0.0)
    }
}
