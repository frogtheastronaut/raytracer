use crate::{camera::Camera, vec3::Vec3, ray::Ray, utils::*, scene::Scene};
use image::RgbImage;

pub fn render<S: Scene>(
    width: u32,
    height: u32,
    camera: &Camera,
    scene: &S
) -> RgbImage {
    let mut img = RgbImage::new(width, height);

    for j in 0..height {
        for i in 0..width {
            let u = i as f64 / (width as f64);
            let v = (height - j - 1) as f64 / (height as f64);

            let ray = camera.get_ray(u, v);

            let pixel_color = if let Some(hit) = scene.hit(&ray) {
                (hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5
            } else {
                scene.background_color(&ray)
            };

            img.put_pixel(i, j, to_rgb(pixel_color));
        }
    }

    img
}
