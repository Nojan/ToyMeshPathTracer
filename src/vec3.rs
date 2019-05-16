use crate::random::*;
use core::ops;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3 {
    data: [f32; 3],
}

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3 { data: [0f32; 3] }
    }

    pub fn fill(x: f32) -> Vec3 {
        Vec3 { data: [x; 3] }
    }

    pub fn min_value() -> Vec3 {
        Vec3::fill(std::f32::NEG_INFINITY)
    }

    pub fn max_value() -> Vec3 {
        Vec3::fill(std::f32::INFINITY)
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { data: [x, y, z] }
    }

    pub fn rand_unit_2d(rng_state: &mut u32) -> Vec3 {
        let mut result = Vec3::zero();
        'length: loop {
            for idx in 0..2 {
                result.data[idx] = (random_float01(rng_state) - 0.5) * 2.0;
            }
            if result.length_sq() <= 1.0 {
                break 'length;
            }
        }
        return result;
    }

    pub fn rand_unit(rng_state: &mut u32) -> Vec3 {
        let mut result = Vec3::zero();
        for idx in 0..3 {
            result.data[idx] = (random_float01(rng_state) - 0.5) * 2.0;
        }
        return result.normalize();
    }

    pub fn get(&self, idx: usize) -> f32 {
        self.data[idx]
    }

    pub fn x(&self) -> f32 {
        self.get(0)
    }

    pub fn y(&self) -> f32 {
        self.get(1)
    }

    pub fn z(&self) -> f32 {
        self.get(2)
    }

    pub fn length_sq(&self) -> f32 {
        Vec3::dot(&self, &self)
    }

    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }

    pub fn hmax(&self) -> f32 {
        let mut result = std::f32::NEG_INFINITY;
        for idx in 0..3 {
            result = result.max(self.data[idx]);
        }
        return result;
    }

    pub fn hmin(&self) -> f32 {
        let mut result = std::f32::INFINITY;
        for idx in 0..3 {
            result = result.min(self.data[idx]);
        }
        return result;
    }

    pub fn to_array(self) -> [f32; 3] {
        return self.data;
    }

    pub fn min(&self, b: &Vec3) -> Vec3 {
        let mut result = Vec3::zero();
        for idx in 0..3 {
            result.data[idx] = self.data[idx].min(b.data[idx]);
        }
        return result;
    }

    pub fn max(&self, b: &Vec3) -> Vec3 {
        let mut result = Vec3::zero();
        for idx in 0..3 {
            result.data[idx] = self.data[idx].max(b.data[idx]);
        }
        return result;
    }

    pub fn dot(&self, b: &Vec3) -> f32 {
        let mut result = 0f32;
        for idx in 0..3 {
            result += self.data[idx] * b.data[idx];
        }
        return result;
    }

    pub fn cross(&self, b: &Vec3) -> Vec3 {
        let x = self.y() * b.z() - self.z() * b.y();
        let y = -(self.x() * b.z() - self.z() * b.x());
        let z = self.x() * b.y() - self.y() * b.x();
        Vec3::from([x, y, z])
    }

    pub fn normalize(&self) -> Vec3 {
        *self * (1.0f32 / self.length())
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(array: [f32; 3]) -> Vec3 {
        Vec3::new(array[0], array[1], array[2])
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        let mut result = Vec3 { data: [0f32; 3] };
        for idx in 0..3 {
            result.data[idx] = self.data[idx] + rhs.data[idx];
        }
        return result;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        let mut result = Vec3 { data: [0f32; 3] };
        for idx in 0..3 {
            result.data[idx] = self.data[idx] - rhs.data[idx];
        }
        return result;
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        let mut result = Vec3 { data: [0f32; 3] };
        for idx in 0..3 {
            result.data[idx] = self.data[idx] * rhs;
        }
        return result;
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        let mut result = Vec3 { data: [0f32; 3] };
        for idx in 0..3 {
            result.data[idx] = self.data[idx] * rhs.data[idx];
        }
        return result;
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        let mut result = Vec3 { data: [0f32; 3] };
        for idx in 0..3 {
            result.data[idx] = self.data[idx] / rhs.data[idx];
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp() {
        let zero = Vec3::zero();
        assert_eq!(zero, zero);
        assert_eq!([0f32; 3], zero.to_array());
        let one = Vec3::fill(1.0);
        assert_ne!(zero, one);
    }

    #[test]
    fn ops_add() {
        let zero = Vec3::fill(0.0);
        let one = Vec3::fill(1.0);
        assert_ne!(zero, one);
        assert_eq!(zero + one, one);
    }

    #[test]
    fn ops_sub() {
        let zero = Vec3::fill(0.0);
        let one = Vec3::fill(1.0);
        let two = Vec3::fill(2.0);
        assert_ne!(two - zero, one);
        assert_eq!(two - one, one);
    }

    #[test]
    fn ops_mul() {
        let zero = Vec3::fill(0.0);
        let one = Vec3::fill(1.0);
        let two = Vec3::fill(2.0);
        assert_eq!(two * zero, zero);
        assert_eq!(two * one, two);
    }

    #[test]
    fn ops_div() {
        let one = Vec3::fill(1.0);
        let two = Vec3::fill(2.0);
        assert_eq!(two / two, one);
        assert_eq!(two / one, two);
    }

    #[test]
    fn dot() {
        let v1 = Vec3::new(0.0, -2.0, 0.0);
        assert_eq!(Vec3::dot(&v1, &v1), 4.0);
    }

    #[test]
    fn length() {
        let v1 = Vec3::new(0.0, -2.0, 0.0);
        assert_eq!(v1.length(), 2.0);
    }

    #[test]
    fn normalize() {
        let v1 = Vec3::new(0.0, -2.0, 0.0);
        assert_eq!(v1.normalize().length(), 1.0);
    }

    #[test]
    fn cross() {
        let v1 = Vec3::new(0.0, 1.0, 0.0);
        let v2 = Vec3::new(0.0, 0.0, 1.0);
        let v3 = Vec3::new(-1.0, 0.0, 0.0);
        assert_eq!(v3, Vec3::cross(&v2, &v1));
    }

    #[test]
    fn min_max() {
        let v1 = Vec3::new(0.0, 1.0, 2.0);
        let v2 = Vec3::new(-1.0, 10.0, 0.5);
        let v_max = Vec3::max(&v1, &v2);
        let v_min = Vec3::min(&v1, &v2);

        for idx in 0..3 {
            assert_eq!(v_max.data[idx], v1.data[idx].max(v2.data[idx]));
            assert_eq!(v_min.data[idx], v1.data[idx].min(v2.data[idx]));
        }
    }

    #[test]
    fn h_min_max() {
        let v = Vec3::new(-1.0, 10.0, 0.5);
        assert_eq!(-1.0, v.hmin());
        assert_eq!(10.0, v.hmax());
    }
}
