mod utils;

use rand::Rng;

use utils::vectors::{ThreeVector, Color};
use utils::rays::Ray;
use utils::hittable::{HitRecord, Hittable};
use utils::hittablelist::HittableList;
use utils::sphere::Sphere;
use utils::rtweekend;
use utils::camera::Camera;

fn ray_color<T: Hittable>(r: &Ray, world: &T) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0., rtweekend::INFINITY, &mut rec) {
        return ((rec.get_normal() + &ThreeVector::new(1.,1., 1.)) * 0.5).into_color()
    }
    let unit_direction = r.get_direction().clone().unit_vector();
    let t = (unit_direction.y + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0)
}
fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 384;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i64 = 100;

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
                pixel_color = pixel_color + ray_color(&r, &world);
            }
            pixel_color.write_color(SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone.");
}