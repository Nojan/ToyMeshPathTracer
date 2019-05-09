use crate::vec3::*;
use crate::ray::*;

pub struct Aabb {
    min: Vec3,
    max: Vec3,
}

impl Aabb {
    pub fn empty() -> Aabb {
        Aabb {min: Vec3::max_value(), max: Vec3::min_value()}
    }

    pub fn is_empty(&self) -> bool {
        for idx in 0..3 {
            if self.min.data[idx] > self.max.data[idx] {
                return true;
            }
        }
        return false;
    }
    
    pub fn contain(&self, point: &Vec3) -> bool {
        if self.is_empty() { return false; }
        for idx in 0..3 {
            let p_idx = point.data[idx];
            if p_idx < self.min.data[idx] { return false; }
            if p_idx > self.max.data[idx] { return false; }
        }
        return true;
    }

    pub fn extend(&mut self, point: &Vec3) {
        if self.is_empty() {
            self.min = *point;
            self.max = *point;
        } else {
            self.min = min(&self.min, &point);
            self.max = max(&self.max, &point);
        }
    }

    pub fn union_aabb(a: &Aabb, b: &Aabb) -> Aabb {
        if a.is_empty() {
            Aabb { min: b.min, max: b.max }
        } else {
            Aabb { min: min(&a.min, &b.min), max: max(&a.max, &b.max) }
        }
    }

    pub fn test_intersection(&self, ray: &Ray, tmin: f32, tmax: f32) -> bool {
        let t0 = (self.min - ray.origin) / ray.dir;
        let t1 = (self.max - ray.origin) / ray.dir;

        let tsmaller = min(&t0, &t1);
        let tbigger = max(&t0, &t1);
        
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
    fn extend_point() {
        let v = Vec3::zero();
        let mut aabb = Aabb::empty();
        assert!(!aabb.contain(&v));
        aabb.extend(&v);
        assert!(aabb.contain(&v));
        aabb.extend(&Vec3::fill(1.0));
        assert!(aabb.contain(&v));
    }

    #[test]
    fn union_aabb() {
        let v0 = Vec3::fill(-1.0);
        let v1 = Vec3::fill(1.0);
        let mut aabb0 = Aabb::empty();
        aabb0.extend(&v0);
        assert!(aabb0.contain(&v0));
        assert!(!aabb0.contain(&v1));
        let mut aabb1 = Aabb::empty();
        aabb1.extend(&v1);
        assert!(!aabb1.contain(&v0));
        assert!(aabb1.contain(&v1));

        let aabb = Aabb::union_aabb(&aabb0, &aabb1);
        assert!(aabb.contain(&v0));
        assert!(aabb.contain(&v1));
    }
    
    #[test]
    fn ray() {
        let v0 = Vec3::fill(-1.0);
        let v1 = Vec3::fill(1.0);
        let mut aabb = Aabb::empty();
        aabb.extend(&v0);
        aabb.extend(&v1);

        let ray_inside = Ray { origin: Vec3::zero(), dir: Vec3::new(0.0, 0.0, 1.0) };
        assert!(aabb.test_intersection(&ray_inside, 0.0, 100.0));

        let ray = Ray { origin: Vec3::new(0.0, 0.0, -5.0), dir: Vec3::new(0.0, 0.0, 1.0) };
        assert!(aabb.test_intersection(&ray, 0.0, 100.0));

        let ray = Ray { origin: Vec3::new(0.0, 0.0, -5.0), dir: Vec3::new(0.0, 1.0, 0.0) };
        assert!(!aabb.test_intersection(&ray, 0.0, 100.0));
    }
}
