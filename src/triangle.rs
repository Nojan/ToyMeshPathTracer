use crate::hit::Hit;
use crate::ray::*;
use crate::vec3::*;

pub struct Triangle {
    pub vertices: [Vec3; 3],
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Triangle {
        Triangle {
            vertices: [v0, v1, v2],
        }
    }

    pub fn intersect(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<Hit> {
        let edge0 = self.vertices[1] - self.vertices[0];
        let edge1 = self.vertices[2] - self.vertices[1];
        let normal = Vec3::cross(&edge0, &edge1).normalize();
        let plane_offset = Vec3::dot(&self.vertices[0], &normal);

        let p0 = ray.point_at(tmin);
        let p1 = ray.point_at(tmax);

        let offset0 = Vec3::dot(&p0, &normal);
        let offset1 = Vec3::dot(&p1, &normal);

        if (offset0 - plane_offset) * (offset1 - plane_offset) <= 0.0 {
            let t = tmin + (tmax - tmin) * (plane_offset - offset0) / (offset1 - offset0);
            let p = ray.point_at(t);

            let c0 = Vec3::cross(&edge0, &(p - self.vertices[0]));
            let c1 = Vec3::cross(&edge1, &(p - self.vertices[1]));
            if Vec3::dot(&c0, &c1) >= 0.0 {
                let edge2 = self.vertices[0] - self.vertices[2];
                let c2 = Vec3::cross(&edge2, &(p - self.vertices[2]));
                if Vec3::dot(&c1, &c2) >= 0.0 {
                    let hit = Hit {
                        pos: p,
                        normal: normal,
                        t: t,
                    };
                    return Some(hit);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_intersection() {
        let triangle = Triangle::new(
            Vec3::new(-0.5, -0.5, 0.0),
            Vec3::new(0.0, 0.5, 0.0),
            Vec3::new(0.5, -0.5, 0.0),
        );
        let ray_not_intersect = Ray::new(&Vec3::new(0.0, 0.0, -0.5), &Vec3::new(0.0, 0.0, -1.0));
        assert!(triangle.intersect(&ray_not_intersect, 0.0, 1.0).is_none());
        let ray_intersect = Ray::new(&ray_not_intersect.origin(), &Vec3::new(0.0, 0.0, 1.0));
        let hit_result = triangle.intersect(&ray_intersect, 0.0, 1.0);
        assert!(hit_result.is_some());
        let hit = hit_result.unwrap();
        assert!((0.5 - hit.t).abs() < 0.001);

        let ray_not_intersect = Ray::new(&Vec3::new(0.5, 0.5, -0.5), &Vec3::new(0.0, 0.0, 1.0));
        assert!(triangle.intersect(&ray_not_intersect, 0.0, 1.0).is_none());
    }
}
