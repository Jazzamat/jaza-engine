use tuples::Tuple;
use color::Color;

const LINE_SIZE: usize = 70;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas{width, height, pixels: vec![vec![Color::new(0.0,0.0,0.0);width];height]}
    }
}

pub fn write_pixel(canvas: &mut Canvas, x: usize,y: usize, color: Color) {
      canvas.pixels[y][x] = color;  
}

pub fn canvas_to_ppm_header(canvas: &mut Canvas) -> String {
    let mut buf = String::new();
    buf.push_str("P3\n"); 
    buf.push_str(format!("{0} {1}\n", canvas.width, canvas.height).as_str());
    buf.push_str("255\n");
    return buf;
} 

pub fn scale_pixel(value: f32, max: usize) -> usize {
    if value >= 1.0 { return max }
    else if value <= 0.0 { return 0 }
    else { return (value * (max as f32)).ceil() as usize};   
}

pub fn canvas_to_ppm(canvas: &mut Canvas) -> String {
    let mut buf = canvas_to_ppm_header(canvas);
    let mut split: usize = 0;
    let mut pixel_string = String::new();
    let mut roll_over_string = String::new();
    for y in 0..canvas.height {
        for x in 0..canvas.width {
        
            buf.push_str(roll_over_string.as_str());
            pixel_string = format!("{0} {1} {2}", 
                scale_pixel(canvas.pixels[y][x].red(), 255),
                scale_pixel(canvas.pixels[y][x].green(), 255),
                scale_pixel(canvas.pixels[y][x].blue(), 255),
            );
            split = split + pixel_string.chars().count();
            if split >= LINE_SIZE {
                let roll_over_len = split - LINE_SIZE;

                println!("Split counter is at :{split}\n");

                println!("Print the pixel sting:");
                println!("{pixel_string}\n");

                roll_over_string = pixel_string.split_off(pixel_string.len() - roll_over_len);
                roll_over_string.push(' ');
                println!("Print the roll over string");
                println!("{roll_over_string}\n");
                buf.push('\n');
                split = 0;
                continue;
            } else {
               roll_over_string.clear(); 
                
            }
            buf.push_str(&pixel_string);         
            if x + 1 != canvas.width {buf.push_str(" ")}
        } 
        buf.push('\n');
        split = 0;
    }
    return buf;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_canvas() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        for i in 0..canvas.width - 1 {
            for j in 0..canvas.height - 1 {
                assert_eq!(canvas.pixels[j][i], Color::new(0.0,0.0,0.0));
            }
        }
    }

    #[test]
    fn test_write_pixel() {
        let mut canvas = Canvas::new(10,20);
        let red = Color::new(1.0,0.0,0.0);
        write_pixel(&mut canvas, 2, 3, red);
        assert_eq!(canvas.pixels[3][2], red)
    }

    #[test]
    fn test_constructing_the_ppm_header() {
        let mut canvas = Canvas::new(5,3);
        let ppm = canvas_to_ppm_header(&mut canvas);    
        assert_eq!(ppm ,"P3\n5 3\n255\n")
    }

    #[test]
    fn test_constructing_the_ppm_pixel_data() {
        let mut canvas = Canvas::new(5,3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        write_pixel(&mut canvas, 0, 0, c1);
        write_pixel(&mut canvas, 2, 1, c2);
        write_pixel(&mut canvas, 4, 2, c3);

        let ppm = canvas_to_ppm(&mut canvas);
        assert_eq!(ppm, "P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n");
    }

    #[test]
    fn test_splitting_long_lines_in_ppm() {
        let mut canvas = Canvas::new(10, 2);

        for y in 0..canvas.height {
            for x in 0..canvas.width {
                write_pixel(&mut canvas, x, y, Color::new(1.0, 0.8, 0.6));
            } 
        }

        let ppm = canvas_to_ppm(&mut canvas);
        assert_eq!(ppm, "P3\n10 2\n255\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n");
    }
}
