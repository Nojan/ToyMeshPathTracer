use crate::ray::*;
use crate::vec3::*;

pub struct Aabb {
    min: Vec3,
    max: Vec3,
}

impl Aabb {
    pub fn empty() -> Aabb {
        Aabb {
            min: Vec3::max_value(),
            max: Vec3::min_value(),
        }
    }

    pub fn is_empty(&self) -> bool {
        for idx in 0..3 {
            if self.min.get(idx) > self.max.get(idx) {
                return true;
            }
        }
        return false;
    }

    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    pub fn surface_area(&self) -> f32 {
        let size = self.size();
        2.0 * (size.x() * size.y() + size.x() * size.z() + size.y() * size.z())
    }

    pub fn contain(&self, point: &Vec3) -> bool {
        if self.is_empty() {
            return false;
        }
        for idx in 0..3 {
            let p_idx = point.get(idx);
            if p_idx < self.min.get(idx) {
                return false;
            }
            if p_idx > self.max.get(idx) {
                return false;
            }
        }
        return true;
    }

    pub fn extend(&self, point: &Vec3) -> Aabb {
        if self.is_empty() {
            Aabb {
                min: *point,
                max: *point,
            }
        } else {
            Aabb {
                min: self.min.min(&point),
                max: self.max.max(&point),
            }
        }
    }

    pub fn union(&self, b: &Aabb) -> Aabb {
        if self.is_empty() {
            Aabb {
                min: b.min,
                max: b.max,
            }
        } else {
            Aabb {
                min: Vec3::min(&self.min, &b.min),
                max: Vec3::max(&self.max, &b.max),
            }
        }
    }

    pub fn test_intersection(&self, ray: &Ray, tmin: f32, tmax: f32) -> bool {
        let t0 = (self.min - ray.origin()) * ray.dir_inv();
        let t1 = (self.max - ray.origin()) * ray.dir_inv();

        let tsmaller = Vec3::min(&t0, &t1);
        let tbigger = Vec3::max(&t0, &t1);

        let tmin = tmin.max(tsmaller.hmax());
        let tmax = tmax.min(tbigger.hmin());

        return tmin <= tmax;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert!(Aabb::empty().is_empty());
    }

    #[test]
    fn size() {
        let v0 = Vec3::fill(-1.0);
        let v1 = Vec3::fill(1.0);
        let aabb = Aabb::empty().extend(&v0).extend(&v1);
        assert_eq!(Vec3::fill(2.0), aabb.size());
    }

    #[test]
    fn extend_point() {
        let v = Vec3::zero();
        let aabb = Aabb::empty();
        assert!(!aabb.contain(&v));
        let aabb = aabb.extend(&v);
        assert!(aabb.contain(&v));
        let aabb = aabb.extend(&Vec3::fill(1.0));
        assert!(aabb.contain(&v));
    }

    #[test]
    fn union_aabb() {
        let v0 = Vec3::fill(-1.0);
        let v1 = Vec3::fill(1.0);
        let aabb0 = Aabb::empty();
        let aabb0 = aabb0.extend(&v0);
        assert!(aabb0.contain(&v0));
        assert!(!aabb0.contain(&v1));
        let aabb1 = Aabb::empty();
        let aabb1 = aabb1.extend(&v1);
        assert!(!aabb1.contain(&v0));
        assert!(aabb1.contain(&v1));

        let aabb = Aabb::union(&aabb0, &aabb1);
        assert!(aabb.contain(&v0));
        assert!(aabb.contain(&v1));
    }

    #[test]
    fn ray() {
        let v0 = Vec3::fill(-1.0);
        let v1 = Vec3::fill(1.0);
        let aabb = Aabb::empty();
        let aabb = aabb.extend(&v0);
        let aabb = aabb.extend(&v1);

        let ray_inside = Ray::new(&Vec3::zero(), &Vec3::new(0.0, 0.0, 1.0));
        assert!(aabb.test_intersection(&ray_inside, 0.0, 100.0));

        let ray = Ray::new(&Vec3::new(0.0, 0.0, -5.0), &Vec3::new(0.0, 0.0, 1.0));
        assert!(aabb.test_intersection(&ray, 0.0, 100.0));

        let ray = Ray::new(&Vec3::new(0.0, 0.0, -5.0), &Vec3::new(0.0, 1.0, 0.0));
        assert!(!aabb.test_intersection(&ray, 0.0, 100.0));
    }
}
