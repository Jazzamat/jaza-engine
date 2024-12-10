const EPSILON: f32 = 0.0001;

use tuples::Tuple;

// TODO Figure out how to do simd operations for efficieny

pub fn float_cmp(a: f32, b:f32) -> bool {
    let delta = a - b;
    if delta.abs() < EPSILON {
        return true;
    } else { 
        return false;
    }
}

// ==================================== MATRIX 4 =================================== // 

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
            // deciding to compare directly here rather than looping to avoid overhead.
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

pub fn multiply_tuple_4(matrix_a: &Matrix4, tuple: &Tuple) -> Tuple {
    let mut result_values = [0.0; 4];
    let a = matrix_a.entries;
    let b = tuple.as_array();

    // hardcoding indices since we know the size and indices
    // row 1
    result_values[0] = a[0]*b[0] + a[1]*b[1] + a[2]*b[2] + a[3]*b[3];
    result_values[1] = a[4]*b[0] + a[5]*b[1] + a[6]*b[2] + a[7]*b[3];
    result_values[2] = a[8]*b[0] + a[9]*b[1] + a[10]*b[2] + a[11]*b[3];
    result_values[3] = a[12]*b[0] + a[13]*b[1] + a[14]*b[2] + a[15]*b[3];

    Tuple::from_values(result_values)
}

pub fn multiply_4(matrix_a: &Matrix4, matrix_b: &Matrix4) -> Matrix4 {
    let mut result_values = [0.0; 16]; // does init all to 0 cost too much?
    let a = matrix_a.entries;
    let b = matrix_b.entries;

    // hardcoding indices since we know the size and indices
    // row 1
    result_values[0] = a[0]*b[0] + a[1]*b[4] + a[2]*b[8] + a[3]*b[12];
    result_values[1] = a[0]*b[1] + a[1]*b[5] + a[2]*b[9] + a[3]*b[13];
    result_values[2] = a[0]*b[2] + a[1]*b[6] + a[2]*b[10] + a[3]*b[14];
    result_values[3] = a[0]*b[3] + a[1]*b[7] + a[2]*b[11] + a[3]*b[15];

    //row 2
    result_values[4] = a[4]*b[0] + a[5]*b[4] + a[6]*b[8] + a[7]*b[12];
    result_values[5] = a[4]*b[1] + a[5]*b[5] + a[6]*b[9] + a[7]*b[13];
    result_values[6] = a[4]*b[2] + a[5]*b[6] + a[6]*b[10] + a[7]*b[14];
    result_values[7] = a[4]*b[3] + a[5]*b[7] + a[6]*b[11] + a[7]*b[15];

    //row 3
    result_values[8] = a[8]*b[0] + a[9]*b[4] + a[10]*b[8] + a[11]*b[12];
    result_values[9] = a[8]*b[1] + a[9]*b[5] + a[10]*b[9] + a[11]*b[13];
    result_values[10] = a[8]*b[2] + a[9]*b[6] + a[10]*b[10] + a[11]*b[14];
    result_values[11] = a[8]*b[3] + a[9]*b[7] + a[10]*b[11] + a[11]*b[15];

    //row 4 
    result_values[12] = a[12]*b[0] + a[13]*b[4] + a[14]*b[8] + a[15]*b[12];
    result_values[13] = a[12]*b[1] + a[13]*b[5] + a[14]*b[9] + a[15]*b[13];
    result_values[14] = a[12]*b[2] + a[13]*b[6] + a[14]*b[10] + a[15]*b[14];
    result_values[15] = a[12]*b[3] + a[13]*b[7] + a[14]*b[11] + a[15]*b[15];

    return Matrix4::new(result_values);
}

pub fn identity_4() -> Matrix4 {
    let mut values = [0.0; 16]; 

    values[0] = 1.0;
    values[5] = 1.0;
    values[10] = 1.0;
    values[15] = 1.0;

    // TODO see if hard coding the whole array would be more performant than individually settting.

    Matrix4::new(values)
}

pub fn transpose_4(matrix: &Matrix4) -> Matrix4 {
    let old_values = matrix.entries;
    let mut new_values = [0.0;16];

    new_values[0] = old_values[0];
    new_values[1] = old_values[4];
    new_values[2] = old_values[8];
    new_values[3] = old_values[12];
    new_values[4] = old_values[1];
    new_values[5] = old_values[5];
    new_values[6] = old_values[9];
    new_values[7] = old_values[13];
    new_values[8] = old_values[2];
    new_values[9] = old_values[6];
    new_values[10] = old_values[10];
    new_values[11] = old_values[14];
    new_values[12] = old_values[3];
    new_values[13] = old_values[7];
    new_values[14] = old_values[11];
    new_values[15] = old_values[15];

    Matrix4::new(new_values)
}

pub fn submatrix_4(matrix: &Matrix4, row:usize, column:usize) -> Matrix3 {
    let mut new_values = [0.0;9];
    
    let (r0,r1,r2) = match row {
        0 => (1,2,3),
        1 => (0,2,3),
        2 => (0,1,3),
        3 => (0,1,2),
        _ => unreachable!()
    };

    let (c0,c1,c2) = match column {
        0 => (1,2,3),
        1 => (0,2,3),
        2 => (0,1,3),
        3 => (0,1,2),
        _ => unreachable!()
    };

    new_values[0] = matrix.get(r0,c0);
    new_values[1] = matrix.get(r0,c1);
    new_values[2] = matrix.get(r0,c2);
    new_values[3] = matrix.get(r1,c0);
    new_values[4] = matrix.get(r1,c1);
    new_values[5] = matrix.get(r1,c2);
    new_values[6] = matrix.get(r2,c0);
    new_values[7] = matrix.get(r2,c1);
    new_values[8] = matrix.get(r2,c2);

    Matrix3::new(new_values)
}

pub fn minor_4(matrix:&Matrix4, row: usize, column:usize) -> f32 {
    0.0 // TODO
}

pub fn cofactor_4(matrix:&Matrix4, row: usize, column:usize) -> f32 {
    0.0 // TODO
}

pub fn determinant_4(matrix: &Matrix4) -> f32 {
    0.0 // TODO
}

// ==================================== MATRIX 3 =================================== // 

#[derive(Debug)]
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

pub fn submatrix_3_match(matrix: &Matrix3, row: usize, column: usize) -> Matrix2 {
    let mut new_values = [0.0; 4];

    let (r0, r1) = match row {
        0 => (1, 2),
        1 => (0, 2),
        2 => (0, 1),
        _ => unreachable!(),
    };

    let (c0, c1) = match column {
        0 => (1, 2),
        1 => (0, 2),
        2 => (0, 1),
        _ => unreachable!(),
    };

    new_values[0] = matrix.get(r0, c0);
    new_values[1] = matrix.get(r0, c1);
    new_values[2] = matrix.get(r1, c0);
    new_values[3] = matrix.get(r1, c1);

    Matrix2::new(new_values)
}

pub fn submatrix_3(matrix: &Matrix3, row: usize, column: usize) -> Matrix2 {
    let mut new_values = [0.0; 4];

    new_values[0] = matrix.get(if row == 0 { 1 } else { 0 }, if column == 0 { 1 } else { 0 });
    new_values[1] = matrix.get(if row == 0 { 1 } else { 0 }, if column == 2 { 1 } else { 2 });
    new_values[2] = matrix.get(if row == 2 { 1 } else { 2 }, if column == 0 { 1 } else { 0 });
    new_values[3] = matrix.get(if row == 2 { 1 } else { 2 }, if column == 2 { 1 } else { 2 });

    Matrix2::new(new_values)
}

pub fn minor_3(matrix: &Matrix3, row: usize, column: usize) -> f32 {
    let submatrix = submatrix_3(matrix, row, column);
    determinant_2(&submatrix)
}

pub fn cofactor_3(matrix: &Matrix3, row: usize, column: usize) -> f32 {
    let minor = minor_3(&matrix, row, column);
    if row + column % 2 == 0 {
        minor
    } else {
        -minor
    }
}

pub fn determinant_3(matrix: &Matrix3) -> f32 {
    0.0
}

// ==================================== MATRIX 2 =================================== // 

#[derive(Debug)]
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

pub fn determinant_2(matrix: &Matrix2) -> f32 {
    let values = matrix.entries;
    values[0]*values[3] - values[1]*values[2]
}

// ==================================== TESTS =================================== // 

#[cfg(test)]
mod tests {

    use std::time::SystemTime;
    use tuples::Tuple;
    use crate::cofactor_3;
    use crate::cofactor_4;
    use crate::determinant_2;
    use crate::determinant_3;
    use crate::determinant_4;
    use crate::identity_4;
    use crate::minor_3;
    use crate::multiply_4;
    use crate::multiply_tuple_4;
    use crate::submatrix_3;
    use crate::submatrix_3_match;
    use crate::submatrix_4;
    use crate::transpose_4;
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

    #[test]
    fn test_mupliply_4() {

        let matrix_a = Matrix4::new([1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,8.0,7.0,6.0,5.0,4.0,3.0,2.0]); 
        let matrix_b = Matrix4::new([-2.0,1.0,2.0,3.0,3.0,2.0,1.0,-1.0,4.0,3.0,6.0,5.0,1.0,2.0,7.0,8.0]); 

        let result = multiply_4(&matrix_a, &matrix_b);

        let expected = Matrix4::new([20.0,22.0,50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0, 26.0, 46.0, 42.0 ]); 

        assert_eq!(result, expected);

    }

    #[test]
    fn test_mupliply_tuple_4() {
        let matrix_a = Matrix4::new([1.0,2.0,3.0,4.0,2.0,4.0,4.0,2.0,8.0,6.0,4.0,1.0,0.0,0.0,0.0,1.0]); 
        let tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let result = multiply_tuple_4(&matrix_a, &tuple);
        let expected = Tuple::from_values([18.0,24.0,33.0,1.0]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mupltiply_identity_matrix_4() {
        let matrix_a = Matrix4::new([0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0,16.0,4.0, 8.0, 16.0, 32.0]); 
        let identity_matrix = identity_4();

        let result = multiply_4(&matrix_a, &identity_matrix);

        assert_eq!(result, Matrix4::new([0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0,16.0,4.0, 8.0, 16.0, 32.0]));
    }

    #[test]
    fn test_mupltiply_identity_matrix_4_with_tuple() {
        let tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let identity_matrix = identity_4();

        let result = multiply_tuple_4(&identity_matrix, &tuple);

        assert_eq!(result, Tuple::new(1.0, 2.0, 3.0, 4.0))
    }

    #[test]
    fn test_transpose() {
        let matrix = Matrix4::new([0.0,9.0,3.0,0.0,9.0, 8.0,0.0,8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0,8.0]);
        let result = transpose_4(&matrix);
        assert_eq!(result, Matrix4::new([0.0,9.0,1.0,0.0,9.0,8.0,8.0,0.0,3.0,0.0,5.0,5.0,0.0,8.0,3.0,8.0]));
    }

    #[test]
    fn test_transpose_identity() {
        let identity_matrix = identity_4();
        let expected = identity_4();
        let result = transpose_4(&identity_matrix);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_determinant_2() {
        let matrix = Matrix2::new([1.0,5.0,-3.0,2.0]);
        let result = determinant_2(&matrix);
        assert_eq!(result, 17.0);
    }

    #[test]
    fn test_submatrix_3() {
        let matrix = Matrix3::new([1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);
        let result = submatrix_3(&matrix, 0, 2);
        let expected = Matrix2::new([-3.0,2.0,0.0,6.0]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_submatrix_3_2() {
        let matrix = Matrix3::new([1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);
        let result = submatrix_3(&matrix, 1, 1);
        let expected = Matrix2::new([1.0,0.0,0.0,-3.0]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_submatrix_3_3() {
        let matrix = Matrix3::new([1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);
        let result = submatrix_3(&matrix, 0, 0);
        let expected = Matrix2::new([2.0,7.0,6.0,-3.0]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_submatrix_4() {
        let matrix = Matrix4::new([-6.0,1.0,1.0,6.0, -8.0,5.0,8.0, 6.0,-1.0,0.0,8.0,2.0,-7.0,1.0,-1.0,1.0]);
        let result = submatrix_4(&matrix, 2, 1);
        let expected = Matrix3::new([-6.0,1.0,6.0,-8.0,8.0,6.0,-7.0,-1.0,1.0]);
        assert_eq!(result, expected);
    }


    #[ignore]
    #[test]
    fn test_submatrix_3_performance() {
        let matrix = Matrix3::new([1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);

        let start = SystemTime::now();
        for i in 0..1000000000 {
            submatrix_3(&matrix, i%3, i%2);
        }
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();

        // match 
        //
        let start_match = SystemTime::now();
        for i in 0..1000000000 {
            submatrix_3_match(&matrix, i%3, i%2);
        }
        let end_match = SystemTime::now();
        let duration_match = end_match.duration_since(start_match).unwrap();


        println!("\n\n TESTING SUBMATRIX PERFORMANCE");
        println!("submatrix_3 took {} milliseconds", duration.as_millis());
        println!("submatrix_3_match took {} milliseconds", duration_match.as_millis());
    } 

    #[test]
    fn test_minor_3() {
        let matrix = Matrix3::new([3.0,5.0,0.0,2.0,-1.0,-7.0, 6.0, -1.0, 5.0]);
        let minor = minor_3(&matrix, 1, 0);
        assert_eq!(minor, 25.0);
    }

    #[test]
    fn test_cofactor_3() {
        let matrix = Matrix3::new([3.0,5.0,0.0,2.0,-1.0,-7.0, 6.0, -1.0, 5.0]);
        let cofactor_a = cofactor_3(&matrix, 0,0);
        let cofactor_b = cofactor_3(&matrix, 1,0);
        assert_eq!(cofactor_a, -12.0);
        assert_eq!(cofactor_b, -25.0);
    }


    #[test]
    fn test_determinant_3() {
        let matrix = Matrix3::new([1.0,2.0,6.0,-5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);
        let cofactor_a = cofactor_3(&matrix, 0, 0);
        let cofactor_b = cofactor_3(&matrix, 0, 1);
        let cofactor_c = cofactor_3(&matrix, 0, 2);
        let determinant = determinant_3(&matrix);

        assert_eq!(cofactor_a, 56.0);
        assert_eq!(cofactor_b, 12.0);
        assert_eq!(cofactor_c, -46.0);
        assert_eq!(determinant, -196.0);
    }

    #[test]
    fn test_determinant_4() {

        let matrix = Matrix4::new([-2.0,-8.0, 3.0, 5.0,-3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0]);
        let cofactor_a = cofactor_4(&matrix, 0, 0);
        let cofactor_b = cofactor_4(&matrix, 0, 1);
        let cofactor_c = cofactor_4(&matrix, 0, 2);
        let cofactor_d = cofactor_4(&matrix, 0, 3);
        let determinant = determinant_4(&matrix);

        assert_eq!(cofactor_a, 690.0);
        assert_eq!(cofactor_b, 447.0);
        assert_eq!(cofactor_c, 210.0);
        assert_eq!(cofactor_d, 51.0);
        assert_eq!(determinant, -4071.0);
    }
}
