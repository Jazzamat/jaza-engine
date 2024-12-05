
pub struct Matrix {
    entries: Vec<f32> // find the most optimal way to have a 2d array in rust
}

impl Matrix {

    pub fn new(values: [f32; 16]) -> Self {
        Matrix{entries:Vec::from(values)}
    } 
    
    pub fn get(&self, row: usize, column: usize) -> f32 {
        self.entries.get(column + row * 4).unwrap().clone()        
    }

}

#[cfg(test)]
mod tests {
    use crate::Matrix;

    #[test]
    fn basic_get() {
        let matrix = Matrix::new([1.0,2.0,3.0,4.0,5.5,6.5,7.5,8.5,9.0,10.0,11.0,12.0,13.5,14.5,15.5,16.5]); 
        assert_eq!(matrix.get(0, 0), 1.0);
        assert_eq!(matrix.get(0, 3), 4.0);
        assert_eq!(matrix.get(1, 0), 5.5);
        assert_eq!(matrix.get(1, 2), 7.5);
        assert_eq!(matrix.get(2, 2), 11.0);
        assert_eq!(matrix.get(3, 0), 13.5);
        assert_eq!(matrix.get(3, 2), 15.5);
    }
}
