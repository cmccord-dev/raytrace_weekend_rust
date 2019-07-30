use crate::Vec3;
pub mod constant_texture;
pub mod checker_texture;
pub mod noise_texture;
pub mod image_texture;
pub trait Texture: std::marker::Sync + std::marker::Send {
    fn value(&self, u:f32, v:f32, p:&Vec3) -> Vec3;
}