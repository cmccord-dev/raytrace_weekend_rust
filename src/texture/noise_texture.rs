use crate::Texture;
use crate::Vec3;
use rand::seq::SliceRandom;
use rand::Rng;

struct Perlin {
    x: Vec<usize>,
    y: Vec<usize>,
    z: Vec<usize>,
    ran_float: Vec<Vec3>,
}

impl Perlin {
    fn new() -> Self {
        Self {
            ran_float: Perlin::generate(),
            x: Perlin::generate_perm(),
            y: Perlin::generate_perm(),
            z: Perlin::generate_perm(),
        }
    }
    fn generate() -> Vec<Vec3> {
        (0..256)
            .map(|_| {
                Vec3::new(
                    -1.0 + 2.0 * rand::thread_rng().gen::<f32>(),
                    -1.0 + 2.0 * rand::thread_rng().gen::<f32>(),
                    -1.0 + 2.0 * rand::thread_rng().gen::<f32>(),
                ).norm()

                //Vec3::from(-1.0 + 2.0 * rand::thread_rng().gen::<f32>())
            })
            .collect::<Vec<Vec3>>()
    }
    fn permute<T>(vec: &mut Vec<T>) {
        vec.shuffle(&mut rand::thread_rng());
    }
    fn generate_perm() -> Vec<usize> {
        let mut p = (0..256).collect::<Vec<usize>>();
        Perlin::permute(&mut p);
        p
    }

    fn noise(&self, p: &Vec3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let (i, j, k) = {
            (p.x.floor() as i32, p.y.floor() as i32, p.z.floor() as i32)
        };
        //println!("{}, {:?}",k, p);
        let c = (0..2)
            .map(|di| {
                (0..2)
                    .map(move |dj| {
                        (0..2)
                            .map(move |dk| {
                                self.ran_float[(self.x[(i + di) as usize & 255]
                                    ^ self.y[(j + dj) as usize & 255]
                                    ^ self.z[(k + dk) as usize & 255])]
                            })
                            .collect::<Vec<Vec3>>()
                    })
                    .collect::<Vec<Vec<Vec3>>>()
            })
            .collect::<Vec<Vec<Vec<Vec3>>>>();
        Perlin::trilinear_interp(c, u, v, w)
    }
    fn trilinear_interp(c: Vec<Vec<Vec<Vec3>>>, u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        c.iter()
            .enumerate()
            .map(|(i, t)| {
                t.iter().enumerate().map(move |(j, t)| {
                    t.iter().enumerate().map(move |(k, c)| {
                        let weight_v = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                        (i as f32 * uu + (1.0 - i as f32) * (1.0 - uu))
                            * (j as f32 * vv + (1.0 - j as f32) * (1.0 - vv))
                            * (k as f32 * ww + (1.0 - k as f32) * (1.0 - ww))
                            * c.dot(&weight_v)
                    })
                })
            })
            .flatten()
            .flatten()
            .sum()
    }
    fn turb(&self, p: &Vec3) -> f32 {
        self.turb_depth(p, 7)
    }
    fn turb_depth(&self, p: &Vec3, depth: u32) -> f32 {
        let mut accum = 0.0;
        let mut temp: Vec3 = *p;
        let mut weight = 1.0;
        (0..depth).for_each(|_| {
            accum += weight * (self.noise(&temp));
            weight *= 0.5;
            temp *= 2.0;
        });
        accum.abs()
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}
impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: &Vec3) -> Vec3 {
        //Vec3::from(1.0) * self.noise.noise(&(p * self.scale))
        let ret = Vec3::from(1.0) * 0.5 * (1.0 + (self.scale*p.z + 10.0 * self.noise.turb(p)).sin());
        //assert!(ret.x > 0.0, "{}", ret.x);
        ret
        /*assert_eq!(ret.x, ret.y);
        assert_eq!(ret.y, ret.z);
        ret*/
    }
}
