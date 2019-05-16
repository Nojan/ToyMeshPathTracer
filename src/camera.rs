use crate::ray::Ray;
use crate::vec3::*;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn look_at(
        look_from: &Vec3,
        look_at: &Vec3,
        up: &Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        _focus_dist: f32,
    ) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = vfov * core::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (*look_from - *look_at).normalize();
        let u = Vec3::cross(&up, &w).normalize();
        let v = Vec3::cross(&w, &u);
        Camera {
            origin: *look_from,
            lower_left_corner: *look_from - u * half_width - v * half_height - w,
            horizontal: u * 2.0 * half_width,
            vertical: v * 2.0 * half_height,
            u: u,
            v: v,
            lens_radius: lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32, state: &mut u32) -> Ray {
        let rd = Vec3::rand_unit_2d(state) * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        let ray_origin = self.origin + offset;
        let ray_normal =
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin;
        Ray::new(&ray_origin, &ray_normal.normalize())
    }
}
