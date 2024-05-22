#![allow(unused_imports)]
use indicatif::ProgressStyle;
use image;
use image::ColorType;
use nalgebra::{Point3, Vector3};

mod math;

use math::ray;

fn ray_color(ray: &ray::Ray) -> Vector3<f64> {
    let unit_direction = ray.get_direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vector3::<f64>::new(1.0, 1.0, 1.0) + t * Vector3::<f64>::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image parameters
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1080;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    //progress Bar
    let pb = indicatif::ProgressBar::new(image_height as u64 * image_width as u64);
    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::<f64>::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::<f64>::new(0.0, viewport_height, 0.0);
    let lower_left_corner_view = -horizontal.clone() / 2.0
        - vertical.clone() / 2.0
        - Vector3::<f64>::new(0.0, 0.0, focal_length);
    let lower_left_corner_world = origin.clone() + lower_left_corner_view;
    let channel = 3;
    // create a one dimensional array of size image_width * image_height * channel
    let mut pixels = vec![0; (image_width * image_height * channel) as usize];
    // fill the array with the color
    for h in 0..image_height {  
        for w in 0..image_width {
            let u = w as f64 / (image_width - 1) as f64;
            let v = (image_height - h - 1) as f64 / (image_height - 1) as f64;
            let p = lower_left_corner_world.clone() + horizontal.clone() * u + vertical.clone() * v;
            let direction = p - origin.clone();
            let ray = ray::Ray::new(origin.clone(), direction);
            let color = ray_color(&ray);
            pixels[w * channel + h * image_width * channel] = (color.x * 255.999) as u8;
            pixels[1 + w * channel + h * image_width * channel] = (color.y * 255.999) as u8;
            pixels[2 + w * channel + h * image_width * channel] = (color.z * 255.999) as u8;
            //progress
            pb.inc(1);
            //show the progress bar after image is generated
            if h == image_height - 1 && w == image_width - 1 {
                pb.finish();
            }
        }
    }
    // save the image
    image::save_buffer(
        "target/output.png",
        &pixels,
        image_width as u32,
        image_height as u32,
        ColorType::Rgb8,
    )
    .unwrap();
}
