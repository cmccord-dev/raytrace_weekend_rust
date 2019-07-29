use crate::Vec3;

pub struct Ray {
    pub src: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray {
            src: a,
            dir: b.norm(),
        }
    }
    pub fn at_parameter(&self, t: f32) -> Vec3 {
        self.src + self.dir * t
    }
}
