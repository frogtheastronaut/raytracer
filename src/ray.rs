
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
	// new ray
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
	// point at parameter t along the ray
	// we use P(t) = origin + t * direction
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}