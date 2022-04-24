mod vector;
mod color;

use std::io::{stderr, Write};

fn main() {
    let image_height = 256;
    let image_width = 256;
    // write header
    print!("P3\n {0} {1} \n255\n", image_width, image_height);

    // image body
    for j in (0..image_height).rev()  {
        stderr().write(b"\x1B[2J\x1B[1;1H").
            expect("error clear");
        stderr().write_fmt(format_args!("Scanlines remaining: {0}\n",j)).
            expect("error printing");

        for i in 0..image_width {
            let color = color::Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25);

            color::Color::println_color(color)
        }
    }

}
