use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use image::Rgb;
use crate::scene::Scene;

pub fn color_from_normal(normal: Vec3) -> Vec3 {
	(normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5
}

pub fn ray_colour(ray: &Ray, sphere: &Sphere) -> Vec3 {
    if let Some(hit) = sphere.hit(ray) {
        return color_from_normal(hit.normal);
    }
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0)*(1.0-t) + Vec3::new(0.5, 0.7, 1.0)*t
}

pub fn to_rgb(color: Vec3) -> Rgb<u8> {
    Rgb([
        (255.999 * color.x) as u8,
        (255.999 * color.y) as u8,
        (255.999 * color.z) as u8,
    ])
}
