use canvas::{canvas_to_ppm, Canvas};
use std::fs;
use color::Color;
use rand::{self, Rng};
use tuples::{create_point, scalar_muplitplication, Tuple};
use engine;

const CANVAS_WIDTH: usize = 2000;
const CANVAS_HEIGHT: usize = 1000;

const OUTPUT_PATH: &str = "/Users/omer/RustTracerChallenge/outputs/debug/output.ppm"; // TODO you can make this
                                                                 // env::home_dir later to support
                                                                 // different platforms
fn main() {
    println!("Hello, world! Creating projectile");
    create_projectile();
}

fn create_projectile() {
    let start_point = tuples::create_point(0.0, 0.0, 0.0);
    let mut velocity = tuples::normalization(&tuples::create_vector(1.0, 1.8, 0.0));
    velocity = scalar_muplitplication(velocity, 11.25);
    
    let mut projectile = engine::Projectile::new(start_point, velocity);

    let gravity = tuples::create_vector(0.0, -0.1, 0.0);
    let wind = tuples::create_vector(-0.01, 0.0, 0.0);
    let environment = engine::Environment::new(gravity, wind);

    let mut my_canvas = Canvas::new(CANVAS_WIDTH,CANVAS_HEIGHT);

    let mut projectile_path: Vec<tuples::Tuple> = Vec::new();

    loop {
        projectile = engine::tick(&environment, &projectile); 

        let pos = projectile.get_position();
        print!("projectile {pos:?}\n\n");

        projectile_path.push(projectile.get_position());
        if tuples::is_point_at_or_below_ground(&projectile.get_position()) {
            break;
        }
    }

    let black = Color::new(0.0, 0.0, 0.0);
    let red = Color::new(1.0, 0.0, 0.0);


    for x in 0..CANVAS_WIDTH {
        for y in 0..CANVAS_HEIGHT {
            if projectile_path.iter().any( 
                |&tuple| tuple.x().round() as usize ==  x && tuple.y().round() as usize == y  
            ) {
                canvas::write_pixel(&mut my_canvas, x, y, red)
            } else {
                canvas::write_pixel(&mut my_canvas, x, y, black)
            }
        }
    }

    let ppm = canvas_to_ppm(&mut my_canvas);
    fs::write(OUTPUT_PATH, ppm).expect("Unable to write to file");
}

fn within_range(a:usize, b:usize) -> bool {
    if a.abs_diff(b) > 2 {return true}
    return false;

}

fn write_random_ppm()  {

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
