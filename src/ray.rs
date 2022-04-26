use crate::point::Point3;
use crate::vector::Vector3;

pub(crate) struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }


}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            origin: Point3::default(),
            direction: Vector3::default(),
        }
    }
}
