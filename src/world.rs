use crate::scene::{Scene, HitRecord};
use crate::ray::Ray;
use crate::vec3::Vec3;

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
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_t = f64::INFINITY;

        for obj in &self.objects {
            if let Some(hit) = obj.hit(ray) {
                if hit.t < closest_t {
                    closest_t = hit.t;
                    closest_hit = Some(hit);
                }
            }
        }

        closest_hit
    }

    fn background_color(&self, ray: &Ray) -> Vec3 {
        Vec3::new(0.5, 0.7, 1.0) // simple background
    }
}
