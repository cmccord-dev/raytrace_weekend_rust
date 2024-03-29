use crate::Texture;
use crate::Vec3;

pub struct ConstantTexture {
    color:Vec3,
}

impl ConstantTexture {
    pub fn new(color:Vec3) -> Self {
        Self {
            color
        }
    }
}
impl Texture for ConstantTexture {
    fn value(&self, _u:f32, _v:f32, _p:&Vec3) -> Vec3 {
        self.color
    }
}