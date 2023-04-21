pub mod pixel;
pub mod ppm_writer;
pub mod color;
pub mod vec3;
pub mod ray;

use pixel::Pixel;
use vec3::{Vec3, Vec3Type};
use ray::Ray;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0, Vec3Type::Point);
    let horizontal = Vec3::new(viewport_width as f32, 0.0, 0.0, Vec3Type::Point);
    let vertical = Vec3::new(0.0, viewport_height as f32, 0.0, Vec3Type::Point);
    let lower_left_corner = origin.clone().sub(horizontal.clone().div_scalar(2.0)).unwrap().sub(vertical.clone().div_scalar(2.0)).unwrap().sub(Vec3::new(0.0, 0.0, focal_length, Vec3Type::Point)).unwrap();

    let mut pixels: Vec<Vec3> = Vec::new();

    for j in (0..=image_height-1).rev() {
        println!("\rScanlines remaining: {}", j);
        for i in 0..=image_width-1 {
            let u = i as f32 / (image_width as f32 - 1.0);
            let v = j as f32 / (image_height as f32 - 1.0);
            let r = Ray::new(origin.clone(), lower_left_corner.clone().add(horizontal.clone().mul_scalar(u)).unwrap().add(vertical.clone().mul_scalar(v)).unwrap().sub(origin.clone()).unwrap());
            let pixel_color = ray_color(r);
            //let pixel_color: Vec3 = Vec3::new(i as f32 / (image_width - 1) as f32, j as f32 / (image_height - 1) as f32, 0.25, Vec3Type::Color);

            pixels.push(pixel_color);
        }
    }
    //color::write_color(pixel_color);
    println!("Done!");

    ppm_writer::write("image.ppm".to_string(), image_width, image_height, pixels).unwrap();
}

pub fn ray_color(r: Ray) -> Vec3 {
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0, Vec3Type::Color).mul_scalar(1.0 - t).add(Vec3::new(0.5, 0.7, 1.0, Vec3Type::Color).mul_scalar(t)).unwrap()
}