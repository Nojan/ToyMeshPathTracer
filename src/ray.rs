use crate::vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    dir: Vec3,
    dir_inv: Vec3,
}

impl Ray {
    pub fn new(origin: &Vec3, direction: &Vec3) -> Ray {
        assert!((direction.length() - 1.0).abs() < 0.001);
        Ray {
            origin: *origin,
            dir: *direction,
            dir_inv: Vec3::new(
                1.0 / direction.x(),
                1.0 / direction.y(),
                1.0 / direction.z(),
            ),
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    pub fn dir_inv(&self) -> Vec3 {
        self.dir_inv
    }

    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + self.dir * t
    }
}
