
mod utils;
use utils::vectors::{ThreeVector, Color};
use utils::rays::Ray;

fn hit_sphere(center: &ThreeVector, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.get_origin() - center;
    let a = ray.get_direction().length_squared();
    let half_b = ThreeVector::dot(&oc, ray.get_direction());
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt() ) / a
    }
}

fn ray_color(r: &Ray) -> Color {
    let t =  hit_sphere(&ThreeVector::new(0., 0., -1.), 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - ThreeVector::new(0., 0., -1.)).unit_vector();
        return Color::new(n.x + 1., n.y + 1., n.z + 1.) * 0.5
    }
    
    let (r1, g1, b1) = (1., 1., 1.);
    let (r2, g2, b2) = (0.5, 0.7, 1.0);
    let unit_diretion = r.get_direction().clone().unit_vector();
    let t = 0.5 * (unit_diretion.y + 1.0);
    Color::new(r1, g1, b1) * (1.0 - t) + Color::new(r2, g2, b2) * t
}
fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 384;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    println!["P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT];
    
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
    
    let origin = ThreeVector::new(0.0, 0.0, 0.0);
    let horizontal = ThreeVector::new(viewport_width, 0., 0.);
    let vertical = ThreeVector::new(0., viewport_height, 0.);
    let lower_left_corner = &origin - &(&horizontal / 2.0) - &vertical / 2.0 - ThreeVector::new(0., 0., focal_length);
    
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanline remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;

            // I have created an borrowing nightmare ....
            let partial1 = &lower_left_corner + &(&horizontal * u);
            let partial2 = &(&vertical * v) - &origin;
            let direction = &partial1 + &partial2;
            let ray = Ray::new(&origin, &direction);
            let pixel_color = ray_color(&ray);
            
            pixel_color.write_color();
        }
    }

    eprintln!("\nDone.");
}