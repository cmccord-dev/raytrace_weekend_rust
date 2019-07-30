#[macro_use]
extern crate impl_ops;
extern crate image;
//use png::(BitDepth,ColorType);
use crate::raytracer::WIDTH;
use crate::raytracer::HEIGHT;

mod camera;
mod world;
mod material;
mod object;
mod ray;
mod raytracer;
mod vec3;
mod aabb;
mod bvh;
mod texture;
use material::*;
use material::lambertian::*;
use material::metal::*;
use material::dielectric::*;
use object::*;
use object::sphere::*;
use vec3::*;
use ray::*;
use camera::*;
use aabb::*;
use world::*;
use bvh::*;
use texture::*;
use texture::constant_texture::*;
use texture::checker_texture::*;
use texture::noise_texture::*;
use texture::image_texture::*;

fn main() {
    println!("Hello, world!");
    let mut image: Vec<Vec<Vec3>> = vec![vec![Vec3::new(0., 0., 0.); WIDTH]; HEIGHT];

    raytracer::raytrace(&mut image);
    save_to_file(&image, "test.png");
}

fn convert_to_u8_arr(arr: &Vec<Vec<Vec3>>) -> Vec<u8> {
    let mut vec = Vec::with_capacity(WIDTH * HEIGHT * 3);

    arr.iter().for_each(|row| {
        row.iter().map(|elem| elem.sqrt()).for_each(|elem| {
            vec.push((elem.x * 255.99999) as u8);
            vec.push((elem.y * 255.99999) as u8);
            vec.push((elem.z * 255.99999) as u8);
        })
    });
    vec
}

fn save_to_file(img: &Vec<Vec<Vec3>>, filename: &str) {
    //let file = File::create(path).unwrap();
    //let ref mut w = BufWriter::new(file);
    /*let mut encoder = png::Encoder::new(w, WIDTH as u32, HEIGHT as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let data = convert_to_u8_arr(img);
    let data = data.as_slice();
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data).unwrap();*/
    let data = convert_to_u8_arr(img);
    let data = data.as_slice();
    image::save_buffer(filename, data, WIDTH as u32, HEIGHT as u32, image::ColorType::RGB(8)).unwrap();
}
