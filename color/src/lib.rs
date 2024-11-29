use tuples::Tuple;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Color {
    tuple: tuples::Tuple,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        let tuple = Tuple::new(red, green, blue, false);
        Color {tuple}
    }

    pub fn red(&self)-> f32 {
        self.tuple.x()
    }

    pub fn green(&self)-> f32 {
        self.tuple.y()
    }

    pub fn blue(&self)-> f32 {
        self.tuple.z()
    }

    pub fn add(c1: Color, c2: Color) -> Color {
        let tuple = tuples::add(&c1.tuple, &c2.tuple);
        Color{tuple}
    }

    pub fn subtract(c1: Color, c2: Color) -> Color {
        let tuple = tuples::subtract(&c1.tuple, &c2.tuple);
        Color{tuple}
    }

    pub fn scalar_muplitplication(c1: Color, scalar:f32) -> Color{
         Color{tuple: tuples::scalar_muplitplication(c1.tuple, scalar)}
    }

    pub fn blend(c1: Color, c2: Color) -> Color {
        return Color{tuple:tuples::hadamard_product(&c1.tuple, &c2.tuple)};
    }
}

impl<'a, 'b> PartialEq<Color> for Color {
    fn eq(&self, other: &Color) -> bool {
        self.tuple.eq(&other.tuple)
    }
}

pub fn float_cmp(a: f32, b:f32) -> bool {
    let delta = a - b;
    if delta.abs() < 0.0001 {
        return true;
    } else { 
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        let result = 2+2;
        assert_eq!(result, 4);
    }

    #[test]
    fn create_basic_colour() {
        let colour = Color::new(-0.5, 0.4, 1.7);
        assert!(colour.red() == -0.5);
        assert!(colour.green() == 0.4);
        assert!(colour.blue() == 1.7);
    }

    #[test]
    fn add_colours() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let result = Color::add(c1,c2);
        assert!(float_cmp(result.red() , 1.6));
        assert!(float_cmp(result.green(), 0.7));
        assert!(float_cmp(result.blue() , 1.0));
    }

    #[test]
    fn subtract_colours() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let result = Color::subtract(c1,c2);
        assert!(float_cmp(result.red() , 0.2));
        assert!(float_cmp(result.green(), 0.5));
        assert!(float_cmp(result.blue() , 0.5));
    }


    #[test]
    fn scalar_multiply_colors() {
        let c1 = Color::new (0.2, 0.3, 0.4);
        let result = Color::scalar_muplitplication(c1,2.0);
        assert!(float_cmp(result.red() , 0.4));
        assert!(float_cmp(result.green(), 0.6));
        assert!(float_cmp(result.blue() , 0.8));
    }

    #[test]
    fn blend_colours() {
        let c1 = Color::new(1.0,0.2,0.4);
        let c2 = Color::new(0.9,1.0,0.1);
        let result = Color::blend(c1,c2); 
        assert_eq!(result, Color::new(0.9,0.2,0.04))
    }
}
