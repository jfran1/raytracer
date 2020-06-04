
mod utils;
use utils::vectors::{ThreeVector, Color};
use utils::rays::Ray;
use utils::hittable::{HitRecord, Hittable};
use utils::hittablelist::HittableList;
use utils::sphere::Sphere;
use utils::rtweekend;

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

    println!["P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT];
    
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
    
    let origin = ThreeVector::new(0.0, 0.0, 0.0);
    let horizontal = ThreeVector::new(viewport_width, 0., 0.);
    let vertical = ThreeVector::new(0., viewport_height, 0.);
    let lower_left_corner = &origin - &(&horizontal / 2.0) - &vertical / 2.0 - ThreeVector::new(0., 0., focal_length);
    
    let mut world = HittableList::new(Box::<Vec<Sphere>>::new(vec![]));
    world.add(Sphere::new(ThreeVector::new(0., 0., -1.), 0.5));
    world.add(Sphere::new(ThreeVector::new(0., -100.5, -1.), 100.));

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
            let pixel_color = ray_color(&ray, &world);
            
            pixel_color.write_color();
        }
    }

    eprintln!("\nDone.");
}