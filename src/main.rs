use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
#[macro_use]
extern crate impl_ops;
//use png::(BitDepth,ColorType);

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;

mod camera;
mod image;

mod material;
mod object;
mod ray;
mod raytracer;
mod vec3;

use image::Image;
use vec3::Vec3;
fn main() {
    println!("Hello, world!");
    let mut image: Image = vec![vec![Vec3::new(0., 0., 0.); WIDTH]; HEIGHT];

    raytracer::raytrace(&mut image);
    save_to_file(&image, "test.png");
}

fn convert_to_u8_arr(arr: &Image) -> Vec<u8> {
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

fn save_to_file(img: &Image, filename: &str) {
    let path = Path::new(filename);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, WIDTH as u32, HEIGHT as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let data = convert_to_u8_arr(img);
    let data = data.as_slice();
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data).unwrap();
}
