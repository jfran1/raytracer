use crate::utils::vectors::ThreeVector;
use crate::utils::rays::Ray;
use crate::utils::hittable::{HitRecord, Hittable};


pub struct Sphere {
    center: ThreeVector,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.get_origin() - self.get_center();
        let a = r.get_direction().length_squared();
        let half_b = ThreeVector::dot(&oc, r.get_direction());
        let c = oc.length_squared() - self.get_radius()*self.get_radius();
        let discriminant = half_b*half_b - a*c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.update_t(temp);
                rec.update_p(r.at(rec.get_t()));
                let outward_normal = (rec.get_p() - self.get_center()) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                return true
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.update_t(temp);
                rec.update_p(r.at(rec.get_t()));
                let outward_normal = (rec.get_p() - self.get_center()) / self.get_radius();
                rec.set_face_normal(r, &outward_normal);
                return true
            }
        }
        false
    }

}

impl Sphere {
    
    pub fn new(center: ThreeVector, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
        }
    }

    pub fn get_center(&self) -> &ThreeVector {
        &self.center
    }
    pub fn get_radius(&self) -> f64 {
        self.radius
    }

}