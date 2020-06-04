use crate::utils::vectors::{ThreeVector};
use crate::utils::rays::Ray;

pub struct HitRecord {
    p: ThreeVector,
    normal: ThreeVector,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    fn update_t(&mut self, t: f64) {
        self.t = t;
    }
    fn update_normal(&mut self, vec: ThreeVector) {
        self.normal = vec;
    }
    fn update_p(&mut self, vec: ThreeVector) {
        self.p = vec;
    }
    fn get_p(&self) -> &ThreeVector {
        &self.p
    }
    fn get_normal(&self) -> &ThreeVector {
        &self.normal
    }
    fn get_t(&self) -> f64 {
        self.t
    }
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &ThreeVector) {
        self.front_face = ThreeVector::dot(r.get_direction(), outward_normal) < 0.;
        self.normal = if self.front_face {outward_normal * 1.0} else {outward_normal * -1.0};
    }
}


trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

