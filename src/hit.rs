use crate::vec3::Vec3;

pub struct Hit {
    pub pos: Vec3,
    pub normal: Vec3,
    pub t: f32,
}
