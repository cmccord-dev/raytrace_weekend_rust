use image::{GenericImageView};
use crate::Vec3;
use crate::Texture;
use std::sync::Arc;
extern crate num;

pub struct Image {
    data:Vec<Vec<Vec3>>,
    width:usize,
    height:usize,
}

impl Image {
    pub fn new(file: &str) -> Image {
        let img = image::open(file).unwrap();
        let (width, height) = (img.width() as usize, img.height() as usize);
        let mut data = vec![vec![Vec3::from(0.0); width as usize]; height as usize];
        println!("{:?}", img.dimensions());
        println!("{:?}", data.len());
        img.pixels().for_each(|(x, y, pixel)| { 
            data[y as usize][x as usize] = Vec3::new(pixel[0] as f32/255.0, pixel[1] as f32/255.0, pixel[1] as f32/255.0);
        });
        Image {
            data,
            width,
            height
        }
    }
}

pub struct ImageTexture {
    img:Image,
}

impl ImageTexture {
    pub fn new(path:&str) -> Self {
        Self {
            img:Image::new(path)
        }
    }
}
impl Texture for ImageTexture {
    fn value(&self, u:f32, v:f32, _p:&Vec3) -> Vec3 {
        let i: usize = num::clamp((self.img.width as f32*u) as usize, 0, self.img.width-1);
        let j: usize = num::clamp(((1.0-v)*self.img.height as f32) as usize, 0, self.img.height-1);
        self.img.data[j][i]
        //Vec3::new(u,v,0.5)
    }
}
