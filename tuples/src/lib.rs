
const EPSILON: f32 = 0.0001;

#[derive(Debug)]
#[allow(dead_code)]
#[derive(PartialEq)]
pub struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: bool // true if is a point otherwise its a vector
}

impl<'a, 'b> PartialEq<Tuple> for (f32,f32,f32,bool) {
    fn eq(&self, other: &Tuple) -> bool {
        self.0.eq(&other.x) && 
        self.1.eq(&other.y) &&
        self.2.eq(&other.z) && 
        self.3 == other.w 
    }
}

impl<'a, 'b> PartialEq<(f32,f32,f32,bool)> for Tuple {
    fn eq(&self, other: &(f32,f32,f32,bool)) -> bool {
        self.x.eq(&other.0) &&
        self.y.eq(&other.1) &&
        self.z.eq(&other.2) &&
        self.w == other.3
    }
}

pub fn float_cmp(a: f32, b:f32) -> bool {
    let delta = a - b;
    if delta < 0.0001 {
        return true;
    } else { 
        return false;
    }
}

pub fn tuple_cmp(a: &Tuple, b: &Tuple) -> bool {
    let delta_x = a.x - b.x;
    let delta_y = a.y - b.y;
    let delta_z = a.z - b.z;
    let same_type = xand(a.w, b.w);

    let delta_x_good = delta_x < EPSILON;
    let delta_y_good = delta_y < EPSILON;
    let delta_z_good = delta_z < EPSILON;

    return delta_x_good && delta_y_good && delta_z_good && same_type
}

pub fn xand(a: bool, b: bool) -> bool {
    if !a && !b {
        return true;
    } else if !a && b{
        return false;
    } else if a && !b {
        return false;
    } else {
        return true;
    }
}

pub fn create_point(x:f32,y:f32,z:f32) -> Tuple {
    Tuple{x,y,z,w:true} // dumbest thing ever so far
}

pub fn create_vector(x:f32,y:f32,z:f32) -> Tuple {
    Tuple{x,y,z,w:false} 
}

pub fn add(a: &Tuple, b: &Tuple) -> Tuple {
    let result = Tuple {
        x : a.x + b.x,
        y : a.y + b.y,
        z : a.z + b.z,
        w : (a.w || b.w)
    };
    result
}

pub fn subtract(a: &Tuple, b: &Tuple) -> Tuple {
    let result = Tuple {
        x : a.x - b.x,
        y : a.y - b.y,
        z : a.z - b.z,
        w : a.w ^ b.w
        // truth table for vectors and points (XOR)
        // a     b   RESULT
        // 0     0   0
        // 1     0   1
        // 0     1   1
        // 1     1   0 
    };
    result
}

pub fn negate(tuple: Tuple) -> Tuple {
    let result = Tuple {
        x : -(tuple.x),
        y : -(tuple.y),
        z : -(tuple.z),
        w : !tuple.w
    };
    return result;
}

pub fn scalar_muplitplication(tuple: Tuple, scalar: f32) -> Tuple {
    let result = Tuple {
        x: tuple.x * scalar,
        y: tuple.y * scalar,
        z: tuple.z * scalar,
        w: tuple.w && scalar.is_sign_positive(),
    };
    return result;
}

pub fn scalar_division(tuple: Tuple, scalar: f32) -> Tuple { // just because
    let result = Tuple {
        x: tuple.x / scalar,
        y: tuple.y / scalar,
        z: tuple.z / scalar,
        w: tuple.w && scalar.is_sign_positive(),
    };
    return result;
}

pub fn is_point(tuple: &Tuple) -> bool {
    // if last element of tuple is 1 its a point otherwise its a vector
    if tuple.w {
        return true;
    }
    return false
}

pub fn is_vector(tuple: &Tuple) -> bool {
    // if last element of tuple is 1 its a point otherwise its a vector
    if tuple.w {
        return false;
    }
    return true;
}

pub fn magnitude(tuple: &Tuple) -> f32 {
    let number: f32 = tuple.x.powi(2) + tuple.y.powi(2) + tuple.z.powi(2); 
    number.sqrt()
}

pub fn normalization(tuple: &Tuple) -> Tuple {
    let magnitude = magnitude(tuple);
    let result = Tuple {
        x: tuple.x/magnitude,
        y: tuple.y/magnitude,
        z: tuple.z/magnitude,
        w: false
    };
    result
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1_is_point() {
        let tuple = Tuple{ x:4.3, y:-4.2, z:3.1, w:true};
        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert!(tuple.w);
        assert!(is_point(&tuple));
        assert!(!is_vector(&tuple))
    }

    #[test]
    fn test_2_is_not_a_point() {
        let tuple = Tuple{ x:4.3, y:-4.2, z:3.1, w:false};
        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert!(!tuple.w);
        assert!(!is_point(&tuple));
        assert!(is_vector(&tuple))
    }

    #[test]
    fn test_create_point() {
        let point : Tuple = create_point(4.0, -4.0, 3.0);
        assert_eq!(point, (4.0,-4.0,3.0,true));
    }

    #[test]
    fn test_create_vector() {
        let point : Tuple = create_vector(4.0, -4.0, 3.0);
        assert_eq!(point, (4.0,-4.0,3.0, false));
    }

    #[test]
    fn test_addition_of_tuples() {
        let a1 = Tuple{ x:3.0, y:-2.0, z:5.0, w:true};
        let a2 = Tuple{ x:-2.0, y:3.0, z:1.0, w:false};
        let result = add(&a1, &a2);
        assert_eq!(result, (1.0,1.0,6.0,true));
    }

    #[test]
    fn test_subtraction_of_points() {
        let a1 = create_point(3.0, 2.0, 1.0);
        let a2 = create_point(5.0, 6.0, 7.0);
        let result = subtract(&a1, &a2);
        assert_eq!(result, (-2.0,-4.0,-6.0,false));
    }

    #[test]
    fn test_subtraction_of_vector_from_a_point() {
        let p = create_point(3.0, 2.0, 1.0);
        let v = create_vector(5.0, 6.0, 7.0);
        let result = subtract(&p, &v);
        assert_eq!(result,(-2.0,-4.0,-6.0,true));

    }

    #[test]
    fn test_subtraction_of_two_vectors() {
        let v1 = create_vector(3.0, 2.0, 1.0);
        let v2 = create_vector(5.0, 6.0, 7.0);
        let result = subtract(&v1, &v2);
        assert_eq!(result, (-2.0,-4.0,-6.0,false));
    }

    #[test]
    fn test_subtraction_of_vector_from_zero_vector() {
        let zero = create_vector(0.0, 0.0, 0.0);
        let v = create_vector(1.0, -2.0, 3.0);
        let result = subtract(&zero, &v);
        assert_eq!(result, (-1.0,2.0,-3.0,false));
    }

    #[test]
    fn test_negation_of_a_tuple() {
        let a = Tuple {x:1.0, y:-2.0, z:3.0, w:false};
        let result = negate(a);
        assert_eq!(result, (-1.0,2.0,-3.0,true));
    }

    #[test]
    fn test_muplitplying_by_a_scalar() {
        let a = Tuple {x: 1.0, y:-2.0, z:3.0, w:false};
        let result = scalar_muplitplication(a, 3.5);
        assert_eq!(result, (3.5,-7.0, 10.5, false));
    }

    #[test]
    fn test_muplitplying_by_a_scalar_2() {
        let a = Tuple {x: 1.0, y:-2.0, z:3.0, w:false};
        let result = scalar_muplitplication(a, 0.5);
        assert_eq!(result, (0.5,-1.0, 1.5, false));
    }

    #[test]
    fn test_dividing_by_a_scalar() {
        let a = Tuple {x: 1.0, y:-2.0, z:3.0, w:false};
        let result = scalar_division(a, 2.0);
        assert_eq!(result, (0.5,-1.0, 1.5, false));
    }

    #[test]
    fn test_magnitude_1() {
        let v = create_vector(1.0, 0.0, 0.0);
        assert_eq!(magnitude(&v), 1.0);
    }

    #[test]
    fn test_magnitude_2() {
        let v = create_vector(0.0, 1.0, 0.0);
        assert_eq!(magnitude(&v), 1.0);
    }

    #[test]
    fn test_magnitude_3() {
        let v = create_vector(0.0, 0.0, 1.0);
        assert_eq!(magnitude(&v), 1.0);
    }

    #[test]
    fn test_magnitude_4() {
        let v = create_vector(1.0, 2.0, 3.0);
        let fourteen: f32 = 14.0; 
        assert_eq!(magnitude(&v), fourteen.sqrt());
    }

    #[test]
    fn test_magnitude_5() {
        let v = create_vector(-1.0, -2.0, -3.0);
        let fourteen: f32 = 14.0; 
        assert_eq!(magnitude(&v), fourteen.sqrt());
    }


    #[test]
    fn test_normalise_1() {
        let v = create_vector(4.0, 0.0, 0.0);
        let result = normalization(&v);
        assert_eq!(result, create_vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_normalise_2() {
        let v = create_vector(1.0, 2.0, 3.0);
        let result = normalization(&v);
        assert!(tuple_cmp(&result, &create_vector(0.26726, 0.53452, 0.80178)));
    }

    #[test]
    fn test_normalise_3() { // todo make this correct
        let v = create_vector(1.0, 2.0, 3.0);
        let result = normalization(&v);
        let result_magnitude = magnitude(&result);
        assert!(float_cmp(result_magnitude,1.0));
    }
}
