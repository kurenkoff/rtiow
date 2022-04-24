use std::ops::{AddAssign, DivAssign, Index, IndexMut, MulAssign, Neg};

pub struct Vector {
    e: [f64; 3],
}

type Color = Vector;
type Point3 = Vector;

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector{ e: [x, y, z] }
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

        return ls;
    }
}

impl Default for Vector {
    fn default() -> Self {
        Vector{e:[0., 0., 0.]}
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl AddAssign<f64> for Vector {
    fn add_assign(&mut self, rhs: f64) {
        for el in &mut self.e {
            *el += rhs
        }
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        for el in &mut self.e {
            *el *= rhs
        }
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        for el in &mut self.e {
            *el /= rhs
        }
    }
}

impl Index<usize> for Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vector{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector{e: [-self.e[0], -self.e[1], -self.e[2]]}
    }
}