use canvas::Canvas;
use std::fs;
use color::Color;
use rand::{self, Rng};

const OUTPUT_PATH: &str = "/Users/omer/RustTracerChallenge/outputs/debug/output.ppm"; // TODO you can make this
                                                                 // env::home_dir later to support
                                                                 // different platforms
fn main() {
    println!("Hello, world! Writing Random Ppm");
    writeRandomPpm();
}

fn writeRandomPpm()  {

    let mut my_canvas = Canvas::new(100,100);
    let mut random_colour;
    let mut red;
    let mut blue;
    let mut green;

    for y in 0..100 {
        for x in 0..100 {
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
