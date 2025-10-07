use crate::{scene::{Scene, HitRecord}, ray::Ray, vec3::Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Scene for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b*b - 4.0*a*c;

        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / (2.0*a);
            if t > 0.0 {
                let point = ray.at(t);
                let normal = (point - self.center).normalize();
                return Some(HitRecord { t, point, normal });
            }
        }
        None
    }

    fn background_color(&self, _ray: &Ray) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0) // spheres don’t define background
    }
}
