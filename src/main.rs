
mod vec3;
mod ray;
mod sphere;
mod utils;
mod camera;
mod renderer;
mod scene;
mod world;
mod material;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils::*;
use crate::camera::Camera;
use crate::renderer::render;
use crate::world::World;
use crate::scene::Scene;
use crate::material::Material;

use image::{RgbImage, Rgb};

fn main() {
    let mut world = World::new();

    let red = Material { colour: Vec3::new(1.0, 0.0, 0.0) };
    let blue = Material { colour: Vec3::new(0.0, 0.0, 1.0) };

    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, red)));
    world.add(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -2.0), 0.3, blue)));

    let camera = Camera::new();

    let img = render(400, 200, &camera, &world);
    img.save("world.png").unwrap();
    println!("Rendered world.png");
}

