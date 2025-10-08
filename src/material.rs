use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub colour: Vec3,
}

impl Material {
    pub fn new(colour: Vec3) -> Self {
        Self { colour }
    }
}
