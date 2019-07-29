use crate::{Material, Scatter};
use crate::Hit;
use crate::Ray;
use crate::Vec3;
pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal { albedo, fuzz }
    }
}
impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let reflected = ray_in.dir.reflect(&hit.normal);
        let scattered = Ray::new(hit.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        if scattered.dir.dot(&hit.normal) > 0.0 {
            Some(Scatter {
                scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
