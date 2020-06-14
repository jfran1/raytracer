
use crate::utils::rays::Ray;
use crate::utils::vectors::ThreeVector;

pub struct Camera {
    origin: ThreeVector,
    lower_left_corner: ThreeVector,
    horizontal: ThreeVector,
    vertical: ThreeVector,
}

impl Camera {
    pub fn new() -> Camera {
        const ASPECT_RATIO: f64 = 16.0 / 9.0;
        
        let viewport_height = 2.0;
        let viewport_width = ASPECT_RATIO * viewport_height;
        let focal_length = 1.0;
        
        let origin = ThreeVector::new(0.0, 0.0, 0.0);
        let horizontal = ThreeVector::new(viewport_width, 0., 0.);
        let vertical = ThreeVector::new(0., viewport_height, 0.);
        let lower_left_corner = {
            &origin - &(&horizontal / 2.0) 
            - &vertical / 2.0 - ThreeVector::new(0., 0., focal_length)};
        
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = &(&self.lower_left_corner + &(&self.horizontal*u)) + &(&(&self.vertical*v) - &self.origin);
        Ray::new(&self.origin, &direction)
    }
}
