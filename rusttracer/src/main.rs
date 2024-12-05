use canvas::Canvas;
use std::fs;
use color::Color;
use rand::{self, Rng};
use tuples;
use engine;

const CANVAS_WIDTH: usize = 2000;
const CANVAS_HEIGHT: usize = 1000;

const OUTPUT_PATH: &str = "/Users/omer/RustTracerChallenge/outputs/debug/output.ppm"; // TODO you can make this
                                                                 // env::home_dir later to support
                                                                 // different platforms
fn main() {
    println!("Hello, world! Creating projective");
}

fn create_projectile() {
    let start_point = tuples::create_point(0.0, 0.0, 0.0);
    let velocity = tuples::normalization(&tuples::create_vector(1.0, 1.8, 0.0));
    let projectile = engine::Projectile::new(start_point, velocity);

    let gravity = tuples::create_vector(0.0, -0.1, 0.0);
    let wind = tuples::create_vector(-0.01, 0.0, 0.0);
    let environment = engine::Environment::new(gravity, wind);

    let mut my_canvas = Canvas::new(CANVAS_WIDTH,CANVAS_HEIGHT);

}

fn writeRandomPpm()  {

    let mut my_canvas = Canvas::new(CANVAS_WIDTH,CANVAS_HEIGHT);
    let mut random_colour;
    let mut red;
    let mut blue;
    let mut green;

    for x in 0..CANVAS_WIDTH {
            for y in 0..CANVAS_HEIGHT {
            red = rand::thread_rng().gen_range(0.0..1.0);
            green = rand::thread_rng().gen_range(0.0..1.0);
            blue = rand::thread_rng().gen_range(0.0..1.0);
            
            random_colour = Color::new(red, green, blue);
            canvas::write_pixel(&mut my_canvas, x, y, random_colour);
        }
    }

    let ppm = canvas::canvas_to_ppm(&mut my_canvas);
    fs::write(OUTPUT_PATH, ppm).expect("Unable to write to file");

}
