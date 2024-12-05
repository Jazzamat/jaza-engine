const EPSILON: f32 = 0.0001;

pub fn float_cmp(a: f32, b:f32) -> bool {
    let delta = a - b;
    if delta.abs() < EPSILON {
        return true;
    } else { 
        return false;
    }
}

#[derive(Debug)]
pub struct Matrix4 {
    entries: [f32;16] // find the most optimal way to have a 2d array in rust
}

impl Matrix4 {
    pub fn new(values: [f32; 16]) -> Self {
        Matrix4{entries:values}
    } 
    
    pub fn get(&self, row: usize, column: usize) -> f32 {
        self.entries[column + row * 4]        
    }


}

impl PartialEq<Matrix4> for Matrix4 {
        fn eq(&self, other: &Matrix4) -> bool {
        float_cmp(self.entries[0], other.entries[0]) && 
        float_cmp(self.entries[1], other.entries[1]) && 
        float_cmp(self.entries[2], other.entries[2]) && 
        float_cmp(self.entries[3], other.entries[3]) && 
        float_cmp(self.entries[4], other.entries[4]) && 
        float_cmp(self.entries[5], other.entries[5]) && 
        float_cmp(self.entries[6], other.entries[6]) && 
        float_cmp(self.entries[7], other.entries[7]) && 
        float_cmp(self.entries[8], other.entries[8]) && 
        float_cmp(self.entries[9], other.entries[9]) && 
        float_cmp(self.entries[10], other.entries[10]) && 
        float_cmp(self.entries[11], other.entries[11]) && 
        float_cmp(self.entries[12], other.entries[12]) && 
        float_cmp(self.entries[13], other.entries[13]) && 
        float_cmp(self.entries[14], other.entries[14]) && 
        float_cmp(self.entries[15], other.entries[15])
        }
} 


pub struct Matrix3 {
    entries: [f32;9] // find the most optimal way to have a 2d array in rust
}

impl Matrix3 {
    pub fn new(values: [f32; 9]) -> Self {
        Matrix3{entries:values}
    } 
    
    pub fn get(&self, row: usize, column: usize) -> f32 {
        self.entries[column + row * 3]        
    }
}

impl PartialEq<Matrix3> for Matrix3 {
        fn eq(&self, other: &Matrix3) -> bool {
        float_cmp(self.entries[0], other.entries[0]) && 
        float_cmp(self.entries[1], other.entries[1]) && 
        float_cmp(self.entries[2], other.entries[2]) && 
        float_cmp(self.entries[3], other.entries[3]) && 
        float_cmp(self.entries[4], other.entries[4]) && 
        float_cmp(self.entries[5], other.entries[5]) && 
        float_cmp(self.entries[6], other.entries[6]) && 
        float_cmp(self.entries[7], other.entries[7]) && 
        float_cmp(self.entries[8], other.entries[8]) 
        }
} 


pub struct Matrix2 {
    entries: [f32;4] // find the most optimal way to have a 2d array in rust
}

impl Matrix2 {
    pub fn new(values: [f32; 4]) -> Self {
        Matrix2{entries:values}
    } 
    
    pub fn get(&self, row: usize, column: usize) -> f32 {
        self.entries[column + row * 2]        
    }
}

impl PartialEq<Matrix2> for Matrix2 {
        fn eq(&self, other: &Matrix2) -> bool {
        float_cmp(self.entries[0], other.entries[0]) && 
        float_cmp(self.entries[1], other.entries[1]) && 
        float_cmp(self.entries[2], other.entries[2]) && 
        float_cmp(self.entries[3], other.entries[3]) 
        }
} 



#[cfg(test)]
mod tests {
    use crate::Matrix4;
    use crate::Matrix3;
    use crate::Matrix2;

    #[test]
    fn basic_get_4() {
        let matrix = Matrix4::new([1.0,2.0,3.0,4.0,5.5,6.5,7.5,8.5,9.0,10.0,11.0,12.0,13.5,14.5,15.5,16.5]); 
        assert_eq!(matrix.get(0, 0), 1.0);
        assert_eq!(matrix.get(0, 3), 4.0);
        assert_eq!(matrix.get(1, 0), 5.5);
        assert_eq!(matrix.get(1, 2), 7.5);
        assert_eq!(matrix.get(2, 2), 11.0);
        assert_eq!(matrix.get(3, 0), 13.5);
        assert_eq!(matrix.get(3, 2), 15.5);
    }

    #[test]
    fn basic_get_2() {
        let matrix = Matrix2::new([-3.0, 5.0, 1.0, -2.0]); 
        assert_eq!(matrix.get(0, 0), -3.0);
        assert_eq!(matrix.get(0, 1), 5.0);
        assert_eq!(matrix.get(1, 0), 1.0);
        assert_eq!(matrix.get(1, 1), -2.0);
    }

    #[test]
    fn basic_get_3() {
        let matrix = Matrix3::new([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]); 
        assert_eq!(matrix.get(0, 0), -3.0);
        assert_eq!(matrix.get(1, 1), -2.0);
        assert_eq!(matrix.get(2, 2), 1.0);
    }


    #[test]
    fn test_equality_4() {
        let matrix_a = Matrix4::new([1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,8.0,7.0,6.0,5.0,4.0,3.0,2.0]); 
        let matrix_b = Matrix4::new([1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,8.0,7.0,6.0,5.0,4.0,3.0,2.0]); 
        assert_eq!(matrix_a, matrix_b);
    }

    #[test]
    fn test_inequality_4() {
        let matrix_a = Matrix4::new([1.0,2.0,1.0,4.0,5.0,6.0,7.0,8.0,2.0,8.0,7.0,6.0,5.0,4.0,3.0,2.0]); 
        let matrix_b = Matrix4::new([1.0,2.0,3.0,4.0,0.0,6.0,7.0,8.0,9.0,8.0,7.0,6.0,5.0,4.0,3.0,2.0]); 
        assert_ne!(matrix_a, matrix_b);
    }


}
