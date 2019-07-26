use crate::material::{Material, Scatter};
use crate::object::Hit;
use crate::ray::Ray;
use crate::vec3::Vec3;
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let target = hit.p + hit.normal + Vec3::random_in_unit_sphere();
        Some(Scatter {
            scattered: Ray::new(hit.p, target - hit.p),
            attenuation: self.albedo,
        })
    }
}
