mod utils;

use utils::vectors::{ThreeVector, Color};
use utils::rays::Ray;
use utils::hittable::{HitRecord, Hittable};
use utils::hittablelist::HittableList;
use utils::sphere::Sphere;
use utils::rtweekend;
use utils::camera::Camera;

fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: i64) -> Color {
    let mut rec = HitRecord::new();
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {return Color::new(0., 0., 0.)}
    if world.hit(r, 0.001, rtweekend::INFINITY, &mut rec) {
        let target = &(&(rec.get_p() + rec.get_normal()) + &ThreeVector::random_in_unit_vector());
        return ray_color(&Ray::new(rec.get_p(), &(target - rec.get_p())), world, depth-1) 
    }
    let unit_direction = r.get_direction().clone().unit_vector();
    let t = (unit_direction.y + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0)*t
}
fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 384; 
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32; 
    const SAMPLES_PER_PIXEL: i64 = 100;
    const MAX_DEPTH: i64 = 50; 

    println!["P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT];
    
    let cam = Camera::new();
    
    let mut world = HittableList::new(Box::<Vec<Sphere>>::new(vec![]));
    world.add(Sphere::new(ThreeVector::new(0., 0., -1.), 0.5));
    world.add(Sphere::new(ThreeVector::new(0., -100.5, -1.), 100.));

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanline remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut  pixel_color = Color::new(0., 0., 0.);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand::random::<f64>()) / (IMAGE_WIDTH -1) as f64; 
                let v = (j as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT -1 ) as f64;
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, MAX_DEPTH);
            }
            pixel_color.write_color(SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone.");
}
