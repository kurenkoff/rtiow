mod color;
mod point;
mod ray;
mod vector;

use std::fs::File;
use std::io::{stderr, Write};

fn ray_color(r: ray::Ray) -> color::Color {
    stderr().write_fmt(format_args!("Ray direction: {}", r.direction)).expect("error debug");
    let unit_direction = vector::Vector3::unit_vector(r.direction);
    stderr().write_fmt(format_args!("Unit Direction: {}", unit_direction)).expect("error debug");
    let t = 0.5 * (unit_direction.y() + 1.0);
    stderr().write_fmt(format_args!("T: {}", t)).expect("error debug");
    stderr().write_fmt(format_args!("\nSource color: {}\n", (1.0 - t) * color::Color::new(1.0, 1.0, 1.0) + t * color::Color::new(0.5, 0.7, 1.0))).expect("error debug");

    (1.0 - t) * color::Color::new(1.0, 1.0, 1.0) + t * color::Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i64 = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    // camera
    let origin = point::Point3::new(0.0, 0.0, 0.0);
    let horizontal = vector::Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = vector::Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - vector::Vector3::new(0.0, 0.0, focal_length);

    // write header
    print!("P3\n {0} {1} \n255\n", image_width, image_height);
    // image body
    for j in (0..image_height).rev() {
        // stderr().write(b"\x1B[2J\x1B[1;1H").expect("error clear");
        stderr()
            .write_fmt(format_args!("Scanlines remaining: {0}\n", j))
            .expect("error printing");

        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            stderr().write_fmt(format_args!("Dir: {}", lower_left_corner + u * horizontal + v * vertical - origin)).expect("error debug");

            let r = ray::Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_color = ray_color(r);
            stderr().write_fmt(format_args!("Color: {}", pixel_color)).expect("error debug");
            color::Color::println_color(pixel_color);
        }
    }
}
