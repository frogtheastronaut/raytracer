use crate::{ray::Ray, vec3::Vec3 ,material::Material};

pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}


pub trait Scene {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;

    fn background_colour(&self, _ray: &Ray) -> Vec3 {
        Vec3::new(0.5, 0.7, 1.0)
    }
}
