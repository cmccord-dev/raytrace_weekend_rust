use crate::{Material, Scatter};
use crate::Hit;
use crate::Ray;
use crate::Vec3;
use crate::Texture;
use std::sync::Arc;
pub struct Lambertian {
    albedo: Arc<Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<Texture>) -> Lambertian {
        Lambertian { albedo:albedo.clone() }
    }
}
impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let target = hit.p + hit.normal + Vec3::random_in_unit_sphere();
        let albedo = self.albedo.value(hit.u,hit.v, &hit.p);
        Some(Scatter {
            scattered: Ray::new(hit.p, target - hit.p),
            attenuation: albedo,
        })
    }
}
