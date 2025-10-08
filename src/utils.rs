use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use image::Rgb;
use crate::scene::Scene;
use crate::material::Material;

pub fn colour_from_normal(normal: Vec3) -> Vec3 {
	(normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5
}

pub fn ray_colour(ray: &Ray, world: &impl Scene) -> Vec3 {
    if let Some(hit) = world.hit(ray) {
        hit.material.colour
    } else {
        // background colour
        Vec3::new(0.5, 0.7, 1.0)
    }
}

pub fn to_rgb(colour: Vec3) -> Rgb<u8> {
    Rgb([
        (255.999 * colour.x) as u8,
        (255.999 * colour.y) as u8,
        (255.999 * colour.z) as u8,
    ])
}
