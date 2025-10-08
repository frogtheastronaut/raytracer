use crate::{ray::Ray, vec3::Vec3, scene::*, material::Material};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t > 0.001 {
                let point = ray.at(t);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord {
                    t,
                    point,
                    normal,
                    material: self.material.clone(),
                });
            }
        }
        None
    }
}

impl Scene for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t > 0.001 {
                let point = ray.at(t);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord {
                    t,
                    point,
                    normal,
                    material: self.material.clone(),
                });
            }
        }
        None
    }
}
