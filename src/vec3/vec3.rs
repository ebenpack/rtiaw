use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn origin() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn unit_vector(v: &Vec3) -> Vec3 {
        let len = v.len();
        *v / len
    }
    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }
    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3::new(
            (v1.y * v2.z) - (v1.z * v2.y),
            (v1.z * v2.x) - (v1.x * v2.z),
            (v1.x * v2.y) - (v1.y * v2.x),
        )
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }
    pub fn len_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.x = self.x + rhs;
        self.y = self.y + rhs;
        self.z = self.z + rhs;
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * &self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * &self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn nearly_equal(a: f64, b: f64) -> bool {
        let abs_a = a.abs();
        let abs_b = b.abs();
        let diff = (a - b).abs();

        if a == b {
            true
        } else if a == 0.0 || b == 0.0 || diff < f64::MIN_POSITIVE {
            diff < (f64::EPSILON * f64::MIN_POSITIVE)
        } else {
            (diff / f64::min(abs_a + abs_b, f64::MAX)) < f64::EPSILON
        }
    }

    #[test]
    fn test_add() {
        let vec1 = Vec3::new(1.0, 1.0, 1.0);
        let vec2 = Vec3::new(10.0, 10.0, 10.0);
        let result = vec1 + vec2;

        assert_eq!(result, Vec3::new(11.0, 11.0, 11.0));

        let vec1 = Vec3::new(1.0, 1.0, 1.0);
        let vec2 = Vec3::new(-1.0, -1.0, -1.0);
        let result = vec1 + (vec2);
        assert_eq!(result, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_subtract() {
        let vec1 = Vec3::new(11.0, 11.0, 11.0);
        let vec2 = Vec3::new(1.0, 1.0, 1.0);
        let result = vec1 - vec2;
        assert_eq!(result, Vec3::new(10.0, 10.0, 10.0));

        let vec1 = Vec3::new(1.0, 1.0, 1.0);
        let vec2 = Vec3::new(1.0, 1.0, 1.0);
        let result = vec1 - vec2;
        assert_eq!(result, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_len() {
        let vec = Vec3::new(1.0, 0.0, 0.0);
        assert_eq!(vec.len(), 1.0);

        let vec = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(vec.len(), 1.0);

        let vec = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(vec.len(), 1.0);

        let vec = Vec3::new(1.0, 1.0, 1.0);
        assert!(nearly_equal(vec.len(), (3.0 as f64).sqrt()));

        let vec = Vec3::new(-1.0, -1.0, -1.0);
        assert!(nearly_equal(vec.len(), (3.0 as f64).sqrt()));

        let vec = Vec3::new(10.0, 10.0, 10.0);
        assert!(nearly_equal(vec.len(), (300.0 as f64).sqrt()));
    }
    #[test]
    fn test_len_squared() {
        let vec = Vec3::new(1.0, 0.0, 0.0);
        assert_eq!(vec.len_squared(), 1.0);

        let vec = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(vec.len_squared(), 1.0);

        let vec = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(vec.len_squared(), 1.0);

        let vec = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(vec.len_squared(), 3.0);

        let vec = Vec3::new(-1.0, -1.0, -1.0);
        assert_eq!(vec.len_squared(), 3.0);

        let vec = Vec3::new(10.0, 10.0, 10.0);
        assert_eq!(vec.len_squared(), 300.0);
    }

    #[test]
    fn test_dot() {
        // let vec1 = Vec3::new(1.0, 1.0, 1.0);
        // let vec2 = Vec3::new(1.0, 1.0, 1.0);
        // let vec3 = Vec3::new(10.0, 10.0, 10.0);
        // let vec4 = Vec3::new(11.0, 11.0, 11.0);
        // let vec5 = Vec3::new(-1.0, -1.0, -1.0);

        let vec1 = Vec3::new(1.0, 1.0, 1.0);
        let vec2 = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(Vec3::dot(&vec1, &vec2), 3.0);

        let vec1 = Vec3::new(1.0, 1.0, 1.0);
        let vec2 = Vec3::new(10.0, 10.0, 10.0);
        assert_eq!(Vec3::dot(&vec1, &vec2), 30.0);

        let vec1 = Vec3::new(10.0, 10.0, 10.0);
        let vec2 = Vec3::new(-1.0, -1.0, -1.0);
        assert_eq!(Vec3::dot(&vec1, &vec2), -30.0);

        let vec1 = Vec3::new(1.0, 0.0, 0.0);
        let vec2 = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(Vec3::dot(&vec1, &vec2), 0.0);

        let vec1 = Vec3::new(1.0, 0.0, 0.0);
        let vec2 = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(Vec3::dot(&vec1, &vec2), 0.0);

        let vec1 = Vec3::new(0.0, 1.0, 0.0);
        let vec2 = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(Vec3::dot(&vec1, &vec2), 0.0);
    }
    #[test]
    fn test_cross() {
        let vec_x = Vec3::new(1.0, 0.0, 0.0);
        let vec_y = Vec3::new(0.0, 1.0, 0.0);
        let vec_z = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!(Vec3::cross(&vec_x, &vec_y), vec_z);
        assert_eq!(Vec3::cross(&vec_y, &vec_z), vec_x);
        assert_eq!(Vec3::cross(&vec_z, &vec_x), vec_y);

        // assert.ok(!vec_y.cross(vec_x).equal(vec_z));
        // assert.ok(!vec_z.cross(vec_y).equal(vec_x));
        // assert.ok(!vec_x.cross(vec_z).equal(vec_y));
        // assert_eq!(vec_x.cross(vec_y).z, 1);
        // assert_eq!(vec_y.cross(vec_z).x, 1);
        // assert_eq!(vec_z.cross(vec_x).y, 1);
        // assert_eq!(t1.x, 1);
        // assert_eq!(t1.y, -5);
        // assert_eq!(t1.z, 3);
    }

    #[test]
    fn test_negate() {
        let vec = Vec3::new(1.0, 1.0, 1.0);
        let neg_vec = Vec3::new(-1.0, -1.0, -1.0);
        assert_eq!(-vec, neg_vec);
        assert_eq!(-(-vec), vec);
    }
}
