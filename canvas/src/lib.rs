use tuples::Tuple;
use color::Color;

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
      canvas.pixels[x][y] = color;  
}

pub fn canvas_to_ppm(canvas: &mut Canvas) -> String {

    let mut buf = String::new();
    buf.push_str("P3\n"); 
    buf.push_str(format!("{0} {1}\n", canvas.width, canvas.height).as_str());
    buf.push_str("255");
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
        assert_eq!(canvas.pixels[2][3], red)
    }

    #[test]
    fn test_constructing_the_ppm_header() {
        let mut canvas = Canvas::new(5,3);
        let ppm = canvas_to_ppm(&mut canvas);    
        assert_eq!(ppm ,"P3\n5 3\n255")
    }
}
