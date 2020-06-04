use crate::utils::vectors;

pub struct Point3 {
    points: vectors::ThreeVector,
}

impl Point3 {
    pub fn new(x: f64, y:f64, z: f64) -> Point3 {
        Point3 {
            points: vectors::ThreeVector::new(x, y, z),
        }
    }
}

pub struct Ray {
    origin: vectors::ThreeVector,
    direction: vectors::ThreeVector,
}

impl Ray {
    pub fn new(origin: &vectors::ThreeVector, direction: &vectors::ThreeVector ) -> Ray {
        Ray{origin: origin.clone(), direction: direction.clone()}
    }
    pub fn get_origin(&self) -> &vectors::ThreeVector {
        &self.origin
    }
    pub fn get_direction(&self) -> &vectors::ThreeVector {
        &self.direction
    }
    pub fn set_origin(&mut self, origin: vectors::ThreeVector) {
        self.origin = origin;
    }
    pub fn set_direction(&mut self, direction: vectors::ThreeVector) {
        self.direction = direction;
    }
    pub fn at(&self, t: f64) -> vectors::ThreeVector {
        self.get_origin() + &(self.get_direction() * t)
    }
}