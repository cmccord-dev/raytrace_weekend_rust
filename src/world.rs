use crate::Dielectric;
use crate::Hit;
use crate::Lambertian;

use crate::Material;
use crate::Metal;
use crate::Object;
use crate::Ray;
use crate::Sphere;
use crate::Vec3;
use crate::AABB;
use rand::Rng;
use std::f32;
use crate::BVH;


pub struct World {
    //pub objs: Vec<Box<Object>>,
    pub bvh : Box<BVH>
}

impl Object for World {
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>{
        /*let mut boxes = self.objs.iter().filter_map(|x| x.bounding_box(t0, t1));
        boxes.next().map(|first| boxes.fold(first, |p,c| {
            p.merge(&c)
        }))*/
        self.bvh.bounding_box(t0, t1)
    }
    fn hits<'a>(&'a self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit<'a>> {
        self.bvh.hits(ray, t_min, t_max)
        /*self.objs
            .iter()
            //.map(|obj| (*obj).hits(ray, 0.01, f32::MAX))
            .filter_map(|obj| (*obj).hits(ray, t_min.max(0.01), t_max))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal))*/
    }
}

impl World {
    pub fn new(objs: Vec<Box<Object>>, t0: f32, t1: f32) -> Self {

        Self { bvh:BVH::new(objs, t0, t1) }
    }
    pub fn build_random_scene(t0:f32, t1:f32) -> World {
        let rand = || rand::thread_rng().gen::<f32>();
        let mut list: Vec<Box<Object>> = (-11..11)
            .flat_map(|a| {
                (-11..11).map(move |b| {
                    let choose = rand();
                    let center =
                        Vec3::new((a as f32) + 0.9 * rand(), 0.2, (b as f32) + 0.9 * rand());
                    if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                        let mat: Box<Material> = if choose < 0.8 {
                            Box::new(Lambertian::new(Vec3::new(
                                rand() * rand(),
                                rand() * rand(),
                                rand() * rand(),
                            )))
                        } else if choose < 0.95 {
                            Box::new(Metal::new(
                                Vec3::new(
                                    0.5 * (1.0 + rand()),
                                    0.5 * (1.0 + rand()),
                                    0.5 * (1.0 + rand()),
                                ),
                                0.5 * rand(),
                            ))
                        } else {
                            Box::new(Dielectric::new(1.5))
                        };
                        let tmp: Box<Object> = Box::new(Sphere::new(center, 0.2, mat));
                        Some(tmp)
                    } else {
                        None
                    }
                })
            })
            .flat_map(|a| a)
            .collect::<Vec<Box<Object>>>();
        list.push(Box::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
        )));
        list.push(Box::new(Sphere::new(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            Box::new(Dielectric::new(1.5)),
        )));
        list.push(Box::new(Sphere::new(
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
        )));
        list.push(Box::new(Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
        )));
        World::new(list, t0, t1)
    }
}
