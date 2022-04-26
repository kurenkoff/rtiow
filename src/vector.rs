use std::fmt::{Display, Formatter, write};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

pub struct Vector3 {
    e: [f64; 3],
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { e: [x, y, z] }
    }

    pub fn x(self) -> f64 {
        self.e[0]
    }

    pub fn y(self) -> f64 {
        self.e[1]
    }

    pub fn z(self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        let mut ls: f64 = 0.;

        for el in self.e {
            ls += el * el;
        }
        println!("LS: {}, Vec: {}", ls, self);
        return ls;
    }

    pub fn dot(lhs: Vector3, rhs: Vector3) -> f64 {
        lhs.e[0] * rhs.e[0] + lhs.e[1] * rhs.e[1] + lhs.e[2] * rhs.e[2]
    }

    pub fn cross(lhs: Vector3, rhs: Vector3) -> Vector3 {
        Vector3 {
            e: [
                lhs.e[1] * rhs.e[2] - lhs.e[2] * rhs.e[1],
                lhs.e[2] * rhs.e[0] - lhs.e[0] * rhs.e[2],
                lhs.e[0] * rhs.e[1] - lhs.e[1] * rhs.e[0],
            ],
        }
    }

    pub fn unit_vector(v: Vector3) -> Vector3 {
        let len = v.length();

        v / len
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Vector3 { e: [0., 0., 0.] }
    }
}

impl Clone for Vector3 {
    fn clone(&self) -> Self {
        Vector3::new(self.e[0], self.e[1], self.e[2])
    }
}

impl Copy for Vector3 {}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        let mut output = Vector3::default();

        output.e[0] = self.e[0] + rhs.e[0];
        output.e[0] = self.e[1] + rhs.e[1];
        output.e[0] = self.e[2] + rhs.e[2];

        output
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl AddAssign<f64> for Vector3 {
    fn add_assign(&mut self, rhs: f64) {
        for el in &mut self.e {
            *el += rhs
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        let mut output = Vector3::default();

        output.e[0] = self.e[0] - rhs.e[0];
        output.e[0] = self.e[1] - rhs.e[1];
        output.e[0] = self.e[2] - rhs.e[2];

        output
    }
}

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut output = Vector3::default();

        output.e[0] = self.e[0] * rhs.e[0];
        output.e[0] = self.e[1] * rhs.e[1];
        output.e[0] = self.e[2] * rhs.e[2];

        output
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut output = Vector3::default();

        output.e[0] = self.e[0] * rhs;
        output.e[0] = self.e[1] * rhs;
        output.e[0] = self.e[2] * rhs;

        output
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        for el in &mut self.e {
            *el *= rhs
        }
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        (1. / rhs) * self
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        for el in &mut self.e {
            *el /= rhs
        }
    }
}

impl Index<usize> for Vector3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.e[0], self.e[1], self.e[2])
    }
}