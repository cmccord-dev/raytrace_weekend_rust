//use std::ops::{Add}; //,AddAssign,Div,DivAssign,Mul,MulAssign,Neg,Sub,SubAssign
use std::ops;
use std::f32;
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }
    pub fn from_value(f: f32) -> Vec3 {
        Vec3 { x: f, y: f, z: f }
    }
    #[inline]
    pub fn dot(self, v: Vec3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
    #[inline]
    pub fn len_sq(self) -> f32 {
        self.dot(self)
    }
    #[inline]
    pub fn len(self) -> f32 {
        self.len_sq().sqrt()
    }
    #[inline]
    pub fn norm(self) -> Vec3 {
        self / self.len()
    }
    #[inline]
    pub fn lerp(a: Vec3, b: Vec3, t: f32) -> Vec3 {
        a * (1.0 - t) + b * t
    }
}
impl From<[f32; 3]> for Vec3 {
    #[inline]
    fn from(v: [f32; 3]) -> Vec3 {
        Vec3::new(v[0], v[1], v[2])
    }
}

impl From<f32> for Vec3 {
    #[inline]
    fn from(v: f32) -> Vec3 {
        Vec3::new(v, v, v)
    }
}

impl_op_ex!(+ |a: &Vec3, b: &Vec3|->Vec3 {Vec3{x:a.x+b.x,y:a.y+b.y,z:a.z+b.z}});
impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
});
impl_op_ex!(*|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x * b.x,
        y: a.y * b.y,
        z: a.z * b.z,
    }
});
impl_op_ex!(/ |a: &Vec3, b: &Vec3|->Vec3 {Vec3{x:a.x/b.x,y:a.y/b.y,z:a.z/b.z}});

impl_op_ex!(+= |a: &mut Vec3, b: &Vec3| {a.x+=b.x;a.y+=b.y;a.z+=b.z;});
impl_op_ex!(-= |a: &mut Vec3, b: &Vec3| {a.x-=b.x;a.y-=b.y;a.z-=b.z;});
impl_op_ex!(*= |a: &mut Vec3, b: &Vec3| {a.x*=b.x;a.y*=b.y;a.z*=b.z;});
impl_op_ex!(/= |a: &mut Vec3, b: &Vec3| {a.x/=b.x;a.y/=b.y;a.z/=b.z;});

impl_op_ex!(+= |a: &mut Vec3, b: f32| {a.x+=b;a.y+=b;a.z+=b;});
impl_op_ex!(-= |a: &mut Vec3, b: f32| {a.x-=b;a.y-=b;a.z-=b;});
impl_op_ex!(*= |a: &mut Vec3, b: f32| {a.x*=b;a.y*=b;a.z*=b;});
impl_op_ex!(/= |a: &mut Vec3, b: f32| {a.x/=b;a.y/=b;a.z/=b;});

impl_op_ex_commutative!(+ |a: Vec3, b: f32|->Vec3 {Vec3{x:a.x+b,y:a.y+b,z:a.z+b}});
impl_op_ex_commutative!(-|a: Vec3, b: f32| -> Vec3 {
    Vec3 {
        x: a.x - b,
        y: a.y - b,
        z: a.z - b,
    }
});
impl_op_ex_commutative!(*|a: Vec3, b: f32| -> Vec3 {
    Vec3 {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
});
impl_op_ex_commutative!(/ |a: Vec3, b: f32|->Vec3 {Vec3{x:a.x/b,y:a.y/b,z:a.z/b}});
/*
impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl std::ops::Add<f32> for Vec3 {
    type Output = Vec3;
    fn add(self, f: f32) -> Vec3 {
        self + Vec3::from_value(f)
    }
}
impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}
impl std::ops::AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, f: f32) {
        *self += Vec3::from_value(f);
    }
}


impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }
}
impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}
impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, f: f32) -> Vec3 {
        self * Vec3::from_value(f)
    }
}
impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, v: Vec3) {
        *self = Self {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }
}
impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, f: f32) {
        *self *= Vec3::from_value(f);
    }
}


impl std::ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / v.x,
            y: self.y / v.y,
            z: self.z / v.z,
        }
    }
}
impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, f: f32) -> Vec3 {
        self / Vec3::from_value(f)
    }
}
impl std::ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, v: Vec3) {
        *self = Self {
            x: self.x / v.x,
            y: self.y / v.y,
            z: self.z / v.z,
        }
    }
}
impl std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, f: f32) {
        *self /= Vec3::from_value(f);
    }
}*/


#[cfg(test)]
mod tests {
    use super::*;
    fn nearly_equal(a: f32, b: f32) -> bool {
        let abs_a = a.abs();
        let abs_b = b.abs();
        let diff = (a - b).abs();
        if a == b {
            // Handle infinities.
            true
        } else if a == 0.0 || b == 0.0 || diff < f32::MIN_POSITIVE {
            // One of a or b is zero (or both are extremely close to it,) use absolute error.
            diff < (f32::EPSILON * f32::MIN_POSITIVE)
        } else {
            // Use relative error.
            (diff / f32::min(abs_a + abs_b, f32::MAX)) < f32::EPSILON
        }
    }
    #[test]
    fn test_add_f32() {
        assert_eq!(Vec3::new(0.0, 0.0, 0.0) + 1.0, Vec3::new(1.0, 1.0, 1.0))
    }
    #[test]
    fn test_add_vec3() {
        assert_eq!(
            Vec3::new(0., 1., 2.) + Vec3::new(1., 2., 3.),
            Vec3::new(1., 3., 5.)
        )
    }
    #[test]
    fn test_addassign_f32() {
        let mut v = Vec3::new(0.0, 0.0, 0.0);
        v += 1.0;
        assert_eq!(v, Vec3::new(1.0, 1.0, 1.0))
    }
    #[test]
    fn test_addassign_vec3() {
        let mut v = Vec3::new(0., 1., 2.);
        v += Vec3::new(1., 2., 3.);
        assert_eq!(v, Vec3::new(1., 3., 5.))
    }
    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        
        assert!(nearly_equal(v1.dot(v2), 20.0));
    }
    #[test]
    fn test_dot_2() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        assert!(nearly_equal(v1.dot(v2), v2.dot(v1)));
    }
    #[test]
    fn test_norm() {
        assert!(nearly_equal(Vec3::new(1.0, 2.0, 3.0).norm().len(), 1.0));
    }
    #[test]
    fn test_len() {
        assert!(nearly_equal(Vec3::new(2.0, 3.0, 6.0).len(), 7.0));
    }
    #[test]
    fn test_len_sqr() {
        assert!(nearly_equal(Vec3::new(2.0, 3.0, 6.0).len_sq(), 49.0));
    }
    #[test]
    fn test_lerp(){
        assert_eq!(Vec3::lerp(Vec3::from([1.0,1.0,1.0]),Vec3::from([0.5,0.7,1.0]),1.0),Vec3::from([0.5,0.7,1.0]));
    }
    #[test]
    fn test_lerp_2(){
        assert_eq!(Vec3::lerp(Vec3::from([1.0,1.0,1.0]),Vec3::from([0.5,0.7,1.0]),0.0),Vec3::from([1.0,1.0,1.0]));
    }
}