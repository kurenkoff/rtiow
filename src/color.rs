use crate::vector::Vector3;

const MULT: f64 = 255.999;

pub type Color = Vector3;


impl Color {
    pub fn println_color(c: Color) {
        let r = (MULT * c.x()) as i64;
        let g = (MULT * c.y()) as i64;
        let b = (MULT * c.z()) as i64;

        print!("{} {} {}\n", r, g, b)
    }
}