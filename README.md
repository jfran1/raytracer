# raytracer

I learn better by doing, so while learning the Rust programming language, I decided to take a stab at following the [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) guide. Converting a program from c++ logic to Rust logic allowed me to encouter many fights against the borrow-check but I know have a better understanding. 

## Installing
### prerequisites
* `rustc`, `cargo`, `rustup`: installation instructions can be found [here](https://www.rust-lang.org/tools/install)

Once the you have rust installed, you can perform the following
1. `git clone https://github.com/jfran1/raytracer.git`
2. `cd raytracer`
3. `cargo build`

## Running
If all things go well then you can generate a ppm image with the command
`cargo run > image.ppm`

This image can be viewed in any ppm viewer (gimp for example).

### Example Output
![output](https://raw.githubusercontent.com/jfran1/raytracer/master/examples/image.png)
