use crate::utils::vectors::{ThreeVector};
use crate::utils::rays::Ray;

pub struct HitRecord {
    p: ThreeVector,
    normal: ThreeVector,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: ThreeVector::zeros(),
            normal: ThreeVector::zeros(),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn update(&mut self, other: &HitRecord) {
        self.p = other.p.clone();
        self.normal = other.normal.clone();
        self.t = other.t;
        self.front_face = other.front_face;
    }

    pub fn update_t(&mut self, t: f64) {
        self.t = t;
    }
    pub fn update_normal(&mut self, vec: ThreeVector) {
        self.normal = vec;
    }
    pub fn update_p(&mut self, vec: ThreeVector) {
        self.p = vec;
    }
    pub fn get_p(&self) -> &ThreeVector {
        &self.p
    }
    pub fn get_normal(&self) -> &ThreeVector {
        &self.normal
    }
    pub fn get_t(&self) -> f64 {
        self.t
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &ThreeVector) {
        self.front_face = ThreeVector::dot(r.get_direction(), outward_normal) < 0.;
        self.normal = if self.front_face {outward_normal * 1.0} else {outward_normal * -1.0};
    }
}


pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

