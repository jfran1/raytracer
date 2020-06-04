use crate::utils::hittable::{Hittable, HitRecord};
use crate::utils::rays::Ray;

pub struct HittableList<T: Hittable> {
    objects: Box::<Vec::<T>>,
}

impl<T: Hittable> HittableList<T> {
    pub fn new(objects: Box::<Vec::<T>>) -> HittableList<T> {
        HittableList {objects}
    }
    pub fn add(&mut self, object: T) {
        self.objects.push(object);
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &(*self.objects) {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.get_t();
                rec.update(&temp_rec);
            }
        }
        hit_anything
    }
}