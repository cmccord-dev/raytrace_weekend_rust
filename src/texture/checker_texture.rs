use crate::Texture;
use crate::Vec3;
use std::sync::Arc;

pub struct CheckerTexture {
    odd:Arc<Texture>,
    even:Arc<Texture>,
}

impl CheckerTexture {
    pub fn new(t0:Arc<Texture>, t1:Arc<Texture>) -> Self {
        Self {
            odd:t1,
            even:t0,
        }
    }
}
impl Texture for CheckerTexture {
    fn value(&self, u:f32, v:f32, p:&Vec3) -> Vec3 {
        let q= 10.0*p;
        let sines = q.x.sin()*q.y.sin()*q.z.sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}