use crate::{ray::Ray, vec3::Vec3};

pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Scene {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
    fn background_color(&self, ray: &Ray) -> Vec3;
}
