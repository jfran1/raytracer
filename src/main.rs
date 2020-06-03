use std::io::{self, Write};
use std::time::Duration;
use std::thread;

mod utils;
use utils::vectors;

fn main() {
    let nx = 256;
    let ny = 256;

    println!["P3\n{} {}\n255", nx, ny];
    for j in (0..ny - 1).rev() {
        for i in 0..nx-1 {
            let color = vectors::Color::new((i-1) as f32/ nx as f32, (j-1) as f32 / ny as f32, 0.25);

            let ir = (255.99 * color.r) as i64;
            let ig = (255.99 * color.g) as i64;
            let ib = (255.99 * color.b) as i64;

            println!["{} {} {}", ir, ig, ib];

        }
    }

}