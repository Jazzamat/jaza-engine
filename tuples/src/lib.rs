const EPSILON: f32 = 0.0001;

#[derive(Debug)]
#[allow(dead_code)]
#[derive(Clone)]
#[derive(Copy)]
/// Data structure that can represent a point or a vector
/// if w is 0 it is a vector
/// if w is 1 it is a point
pub struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32 // true if is a point otherwise its a vector
}

impl Tuple {
    pub fn new(x:f32,y:f32,z:f32,w:f32) -> Self{
        Tuple{x,y,z,w}
    }

    pub fn from_values(values: [f32;4]) -> Self {
        Tuple{x:values[0], y:values[1], z:values[2], w:values[3]}
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn w(&self) -> f32 {
        self.w
    }

    pub fn eq(&self, other: &Tuple) -> bool {
        float_cmp(self.x, other.x) && 
        float_cmp(self.y,other.y) &&
        float_cmp(self.z,other.z) && 
        float_cmp(self.w,other.w)  
    }

    pub fn as_array(&self) -> [f32;4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl<'a, 'b> PartialEq<Tuple> for (f32,f32,f32,f32) {
    fn eq(&self, other: &Tuple) -> bool {
        float_cmp(self.0, other.x) && 
        float_cmp(self.1,other.y) &&
        float_cmp(self.2,other.z) && 
        float_cmp(self.3,other.w)
    }
}

impl<'a,'b> PartialEq<(f32,f32,f32,f32)> for  Tuple{

    fn eq(&self, other: &(f32,f32,f32,f32)) -> bool {
        float_cmp(self.x, other.0) &&
            float_cmp(self.y,other.1) &&
            float_cmp(self.z,other.2) &&
            float_cmp(self.w,other.3)
    }
}

impl<'a,'b> PartialEq<Tuple> for  Tuple{

    fn eq(&self, other: &Tuple) -> bool {
        float_cmp(self.x, other.x) &&
            float_cmp(self.y,other.y) &&
            float_cmp(self.z,other.z) &&
            float_cmp(self.w,other.w)
    }
}

pub fn float_cmp(a: f32, b:f32) -> bool {
    let delta = a - b;
    if delta.abs() < EPSILON {
        true
    } else {
        false
    }
}

pub fn is_point_at_or_below_ground(point: &Tuple) -> bool {
    if is_vector(&point) {panic!("This function needs a point, not vectors");}
    let delta_zero = point.y - 0.0;
    return delta_zero <= EPSILON;
}

pub fn tuple_cmp(a: &Tuple, b: &Tuple) -> bool {
    float_cmp(a.x, b.x) &&
    float_cmp(a.y, b.y) &&
    float_cmp(a.z, b.z) &&
    float_cmp(a.w, b.w)
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
    Tuple{x,y,z,w:1.0} // dumbest thing ever so far // why?
}

pub fn create_vector(x:f32,y:f32,z:f32) -> Tuple {
    Tuple{x,y,z,w:0.0} 
}

pub fn add(a: &Tuple, b: &Tuple) -> Tuple {
    let result = Tuple {
        x : a.x + b.x,
        y : a.y + b.y,
        z : a.z + b.z,
        w : a.w + b.w
    };
    result
}

pub fn subtract(a: &Tuple, b: &Tuple) -> Tuple {
    let result = Tuple {
        x : a.x - b.x,
        y : a.y - b.y,
        z : a.z - b.z,
        w : a.w - b.w
    };
    result
}

pub fn negate(tuple: Tuple) -> Tuple {
    let result = Tuple {
        x : -(tuple.x),
        y : -(tuple.y),
        z : -(tuple.z),
        w : -(tuple.w)
    };
    return result;
}

pub fn dot_product(a:&Tuple, b:&Tuple) -> f32 {
    if a.w == 1.0 || b.w == 1.0 {
        panic!("Can't dot product a point");
    }
    return a.x * b.x +
            a.y * b.y +     
            a.z * b.z
}


pub fn cross_product(a: &Tuple, b: &Tuple) -> Tuple {
    if a.w == 1.0 || b.w == 1.0 {
        panic!("Can't cross product a point -> a is a point?: {aw:?}, b is a point?: {bw:?}", aw = a.w, bw = b.w);
    }
    return create_vector(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x
    )   
}

pub fn scalar_muplitplication(tuple: Tuple, scalar: f32) -> Tuple { // TODO would adjusting the old
    // tuples value be more performant?
    let result = Tuple {
        x: tuple.x * scalar,
        y: tuple.y * scalar,
        z: tuple.z * scalar,
        w: tuple.w * scalar
    };
    return result;
}

pub fn scalar_division(tuple: Tuple, scalar: f32) -> Tuple { // just because
    let result = Tuple {
        x: tuple.x / scalar,
        y: tuple.y / scalar,
        z: tuple.z / scalar,
        w: tuple.w / scalar,

    };
    return result;
}

pub fn hadamard_product(a: &Tuple, b: &Tuple) -> Tuple {
     Tuple::new(
        a.x * b.x,
        a.y * b.y,
        a.z * b.z,
        a.w * b.w
    )
}

pub fn is_point(tuple: &Tuple) -> bool {
    // if last element of tuple is 1 its a point otherwise its a vector
    if tuple.w == 1.0 {
        return true;
    }
    return false
}

pub fn is_vector(tuple: &Tuple) -> bool {
    // if last element of tuple is 1 its a point otherwise its a vector
    if tuple.w == 1.0 {
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
        w: tuple.w/magnitude
    };
    result
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1_is_point() {
        let tuple = Tuple{ x:4.3, y:-4.2, z:3.1, w:1.0};
        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert_eq!(tuple.w, 1.0);
        assert!(is_point(&tuple));
        assert!(!is_vector(&tuple))
    }

    #[test]
    fn test_2_is_not_a_point() {
        let tuple = Tuple{ x:4.3, y:-4.2, z:3.1, w:0.0};
        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert_eq!(tuple.w, 0.0);
        assert!(!is_point(&tuple));
        assert!(is_vector(&tuple))
    }

    #[test]
    fn test_create_point() {
        let point : Tuple = create_point(4.0, -4.0, 3.0);
        assert_eq!(point, (4.0,-4.0,3.0,1.0));
    }

    #[test]
    fn test_create_vector() {
        let point : Tuple = create_vector(4.0, -4.0, 3.0);
        assert_eq!(point, (4.0,-4.0,3.0, 0.0));
    }

    #[test]
    fn test_addition_of_tuples() {
        let a1 = Tuple{ x:3.0, y:-2.0, z:5.0, w:1.0};
        let a2 = Tuple{ x:-2.0, y:3.0, z:1.0, w:0.0};
        let result = add(&a1, &a2);
        assert_eq!(result, (1.0,1.0,6.0,1.0));
    }

    #[test]
    fn test_addition_of_points() {
        let a1 = create_point(3.0, -2.0, 5.0);
        let a2 = create_point(-2.0, 3.0, 1.0);
        let result = add(&a1, &a2);
        assert_eq!(result, (1.0, 1.0, 6.0, 2.0)); // w: 2.0 make sense?
    }

    #[test]
    fn test_addition_of_points_and_vectors() {
        let a1 = create_point(3.0, -2.0, 5.0);
        let a2 = create_vector(-2.0, 3.0, 1.0);
        let result = add(&a1, &a2);
        assert_eq!(result, (1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn test_subtraction_of_points() {
        let a1 = create_point(3.0, 2.0, 1.0);
        let a2 = create_point(5.0, 6.0, 7.0);
        let result = subtract(&a1, &a2);
        assert_eq!(result, (-2.0,-4.0,-6.0,0.0));
    }

    #[test]
    fn test_subtraction_of_vector_from_a_point() {
        let p = create_point(3.0, 2.0, 1.0);
        let v = create_vector(5.0, 6.0, 7.0);
        let result = subtract(&p, &v);
        assert_eq!(result,(-2.0,-4.0,-6.0,1.0));

    }

    #[test]
    fn test_subtraction_of_two_vectors() {
        let v1 = create_vector(3.0, 2.0, 1.0);
        let v2 = create_vector(5.0, 6.0, 7.0);
        let result = subtract(&v1, &v2);
        assert_eq!(result, (-2.0,-4.0,-6.0,0.0));
    }

    #[test]
    fn test_subtraction_of_vector_from_zero_vector() {
        let zero = create_vector(0.0, 0.0, 0.0);
        let v = create_vector(1.0, -2.0, 3.0);
        let result = subtract(&zero, &v);
        assert_eq!(result, (-1.0,2.0,-3.0,0.0));
    }

    #[test]
    fn test_negation_of_a_tuple() {
        let a = Tuple {x:1.0, y:-2.0, z:3.0, w:0.0};
        let result = negate(a);
        assert_eq!(result, (-1.0,2.0,-3.0,0.0));
    }

    #[test]
    fn test_muplitplying_by_a_scalar() {
        let a = Tuple {x: 1.0, y:-2.0, z:3.0, w:0.0};
        let result = scalar_muplitplication(a, 3.5);
        assert_eq!(result, (3.5,-7.0, 10.5, 0.0));
    }

    #[test]
    fn test_muplitplying_by_a_scalar_2() {
        let a = Tuple {x: 1.0, y:-2.0, z:3.0, w:0.0};
        let result = scalar_muplitplication(a, 0.5);
        assert_eq!(result, (0.5,-1.0, 1.5, 0.0));
    }

    #[test]
    fn test_dividing_by_a_scalar() {
        let a = Tuple {x: 1.0, y:-2.0, z:3.0, w:0.0};
        let result = scalar_division(a, 2.0);
        assert_eq!(result, (0.5,-1.0, 1.5, 0.0));
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

    #[test]
    fn dot_product_of_two_tuples() {
        let a = create_vector(1.0, 2.0, 3.0);
        let b = create_vector(2.0, 3.0, 4.0);
        let result = dot_product(&a, &b);
        assert!(float_cmp(result, 20.0))
    }

    #[test]
    #[should_panic]
    fn dot_product_of_vector_and_point() {
        let a = create_vector(1.0, 2.0, 3.0);
        let b = create_point(2.0, 3.0, 4.0);
        dot_product(&a, &b);
    }

    #[test]
    #[should_panic]
    fn dot_product_of_two_points() {
        let a = create_point(1.0, 2.0, 3.0);
        let b = create_point(2.0, 3.0, 4.0);
        dot_product(&a, &b);
    }
    
    #[test]
    fn cross_product_of_two_vectors() {
        let a = create_vector(1.0, 2.0, 3.0);    
        let b = create_vector(2.0, 3.0, 4.0);    
        let result = cross_product(&a, &b);
        let result_prime = cross_product(&b, &a);
        assert!(tuple_cmp(&result, &create_vector(-1.0, 2.0, -1.0)));  
        assert!(tuple_cmp(&result_prime, &create_vector(1.0, -2.0, 1.0)));  
    }

    #[test]
    #[should_panic]
    fn cross_product_of_a_point_and_a_vector() {
        let a = create_vector(1.0, 2.0, 3.0);    
        let b = create_point(2.0, 3.0, 4.0);    
        cross_product(&a, &b);
    }

    #[test]
    #[should_panic]
    fn cross_product_of_a_point_and_a_vector_2() {
        let a = create_point(1.0, 2.0, 3.0);    
        let b = create_vector(2.0, 3.0, 4.0);    
        cross_product(&a, &b);
    }

    #[test]
    #[should_panic]
    fn cross_product_of_two_points() {
        let a = create_point(1.0, 2.0, 3.0);    
        let b = create_point(2.0, 3.0, 4.0);    
        cross_product(&a, &b);
    }

    #[test]
    fn test_hadamard_product() { // TODO why do I need to compare each attribute?
        let a = create_point(1.0, 0.2, 0.4);
        let b = create_point(0.9, 1.0, 0.1);
        let result = hadamard_product(&a, &b);
        assert_eq!(result, (0.9,0.2,0.04, 1.0)); // should w:1.0?
    }

    #[test]
    fn test_is_point_on_the_ground() {
        let point = create_point(0.0, 0.0, 0.0);
        assert!(is_point_at_or_below_ground(&point));
    }

    #[test]
    fn test_is_point_blow_ground() {
        let point = create_point(0.0, -19.0, 0.0);
        assert!(is_point_at_or_below_ground(&point));
    }

    #[test]
    fn test_is_point_is_only_just_above_ground() {
        let point = create_point(0.0, 0.001, 0.0);
        assert!(!is_point_at_or_below_ground(&point));
    }

    #[test]
    fn test_is_point_is_only_just_below_ground() {
        let point = create_point(0.0, -0.001, 0.0);
        assert!(is_point_at_or_below_ground(&point));
    }
}
    
