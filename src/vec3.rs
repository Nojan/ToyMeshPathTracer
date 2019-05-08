use std::ops;
use crate::random::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3 {
    pub data: [f32; 3],
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

    pub fn rand_unit(rng_state: &mut u32) -> Vec3 {
        let mut result = Vec3::zero();
        for idx in 0..3 {
            result.data[idx] = (random_float01(rng_state) - 0.5) + 2.0;
        }
        return normalize(&result);
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }

    pub fn z(&self) -> f32 {
        self.data[2]
    }

    pub fn length_sq(&self) -> f32 {
        dot(&self, &self)
    }

    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }
}

pub fn min(a: &Vec3, b: &Vec3) -> Vec3 {
    let mut result = Vec3::zero();
    for idx in 0..3 {
        result.data[idx] = a.data[idx].min(b.data[idx]);
    }
    return result;
}

pub fn max(a: &Vec3, b: &Vec3) -> Vec3 {
    let mut result = Vec3::zero();
    for idx in 0..3 {
        result.data[idx] = a.data[idx].max(b.data[idx]);
    }
    return result;
}

pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
    let mut result = 0f32;
    for idx in 0..3 {
        result += a.data[idx] * b.data[idx];
    }
    return result;
}

pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
    let x = a.y() * b.z() - a.z() * b.y();
    let y = -(a.x() * b.z() - a.z() * b.x());
    let z = a.x() * b.y() - a.y() * b.x();
    Vec3 { data: [x, y, z] }
}

pub fn normalize(v: &Vec3) -> Vec3 {
    *v * (1.0f32 / v.length())
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
        let zero = Vec3 { data: [0f32; 3] };
        assert_eq!(zero, zero);
        let one = Vec3 { data: [1f32; 3] };
        assert_ne!(zero, one);
    }

    #[test]
    fn ops_add() {
        let zero = Vec3 { data: [0f32; 3] };
        let one = Vec3 { data: [1f32; 3] };
        assert_ne!(zero, one);
        assert_eq!(zero + one, one);
    }

    #[test]
    fn ops_sub() {
        let zero = Vec3 { data: [0f32; 3] };
        let one = Vec3 { data: [1f32; 3] };
        let two = Vec3 { data: [2f32; 3] };
        assert_ne!(two - zero, one);
        assert_eq!(two - one, one);
    }

    #[test]
    fn ops_mul() {
        let zero = Vec3 { data: [0f32; 3] };
        let one = Vec3 { data: [1f32; 3] };
        let two = Vec3 { data: [2f32; 3] };
        assert_eq!(two * zero, zero);
        assert_eq!(two * one, two);
    }

    #[test]
    fn ops_div() {
        let one = Vec3 { data: [1f32; 3] };
        let two = Vec3 { data: [2f32; 3] };
        assert_eq!(two / two, one);
        assert_eq!(two / one, two);
    }

    #[test]
    fn dot() {
        let v1 = Vec3 {
            data: [0.0, -2.0, 0.0],
        };
        assert_eq!(super::dot(&v1, &v1), 4.0);
    }

    #[test]
    fn length() {
        let v1 = Vec3::new(0.0, -2.0, 0.0);
        assert_eq!(v1.length(), 2.0);
    }

    #[test]
    fn normalize() {
        let v1 = Vec3::new(0.0, -2.0, 0.0);
        assert_eq!(super::normalize(&v1).length(), 1.0);
    }

    #[test]
    fn cross() {
        let v1 = Vec3::new(0.0, 1.0, 0.0);
        let v2 = Vec3::new(0.0, 0.0, 1.0);
        let v3 = Vec3::new(-1.0, 0.0, 0.0);
        assert_eq!(v3, super::cross(&v2, &v1));
    }

    #[test]
    fn min_max() {
        let v1 = Vec3::new(0.0, 1.0, 2.0);
        let v2 = Vec3::new(-1.0, 10.0, 0.5);
        let v_max = max(&v1, &v2);
        let v_min = min(&v1, &v2);

        for idx in 0..3 {
            assert_eq!(v_max.data[idx], v1.data[idx].max(v2.data[idx]));
            assert_eq!(v_min.data[idx], v1.data[idx].min(v2.data[idx]));
        }
    }
}
