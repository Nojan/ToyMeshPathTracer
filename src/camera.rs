use crate::ray::Ray;
use crate::vec3::*;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    fn get_ray(&self, s: f32, t: f32, state: &mut u32) -> Ray {
        let rd = Vec3 {
            data: [1.0, 1.0, 1.0],
        };
        let offset = self.u * rd.x() + self.v * rd.y();
        let ray_origin = self.origin + offset;
        let ray_normal =
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset;
        Ray {
            origin: ray_origin,
            dir: normalize(&ray_normal),
        }
    }
}
