use crate::{Material, Scatter};
use crate::Hit;
use crate::Ray;
use crate::Vec3;
use rand::Rng;
pub struct Dielectric {
    ref_idx: f32,
}
fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric { ref_idx }
    }
}
impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let (normal, ni_over_nt, cosine) = if ray_in.dir.dot(&hit.normal) > 0.0 {
            let cosine = ray_in.dir.dot(&hit.normal);
            (
                -hit.normal,
                self.ref_idx,
                (1.0-self.ref_idx*self.ref_idx*(1.0-cosine*cosine)).sqrt()
            )
        } else {
            (hit.normal, 1.0 / self.ref_idx, -ray_in.dir.dot(&hit.normal))
        };
        let refracted = ray_in.dir.refract(&normal, ni_over_nt);
        let reflect_prob = match refracted {
            Some(_) => schlick(cosine, self.ref_idx),
            None => 1.0,
        };
        Some(Scatter {
            scattered: Ray::new(
                hit.p,
                if rand::thread_rng().gen::<f32>() < reflect_prob {
                    ray_in.dir.reflect(&hit.normal)
                } else {
                    refracted.unwrap()
                },
            ),
            attenuation: Vec3::from(1.0),
        })
    }
}
