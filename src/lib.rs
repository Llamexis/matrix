use std::fmt;
use std::ops::{Index, Mul, IndexMut};


#[derive(Debug)]
pub struct Matrix<T: Default + Clone >{
    cols: usize,
    rows: usize,
    data: Box<Vec<T>>,
}

impl<T: Default + Clone > Matrix<T> {
    pub fn new(cols: usize, rows: usize) -> Matrix<T> {
    let  data = Box::new(vec![T::default();cols*rows]);
        Self{
            cols,
            rows,
            data,
        }
    }
    pub fn from_data(cols:usize, rows: usize, data: &[T]) -> Self {
        Self{
            cols,
            rows,
            data: Box::new(Vec::from(data)),
        }
    }

    pub fn dim(&self) -> (usize,usize) {
        (self.cols,self.rows)
    }

}
impl<T: Default + Clone + Copy> Matrix<T> {
    pub fn transpose(&self) -> Matrix<T> {
        let mut d = vec![T::default(); self.cols * self.rows];
        for i in 0..self.rows {
            for j in 0..self.cols {
                d[j * self.rows + i] = self.data[i * self.cols + j];
            }
        }
        Matrix {
            cols: self.rows,
            rows: self.cols,
            data: Box::new(d),
        }
    }
}
impl<T: Default + Clone + Copy + Mul<Output = T>> Matrix<T> {
    pub fn multiply_by_scalar(&mut self, scalar: T) -> &Self {
        for row in 0..self.rows{
            for col in 0..self.cols{
                self.data[col+row*self.cols] = self.data[col+row*self.cols] * scalar;
            }
        }
        self
    }
}

impl<T: fmt::Display + Clone + Default> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{} ", self.data[row * self.cols + col])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T:Default + Clone > Index<(usize,usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, (row,col): (usize,usize)) -> &Self::Output {
        &self.data[row * self.cols + col]
    }
}

impl<T:Default + Clone > IndexMut<(usize,usize)> for Matrix<T> {
    fn index_mut(&mut self, (row,col): (usize,usize)) -> &mut T {
        &mut self.data[row * self.cols + col]
    }
}

impl<T: Default + Clone + Copy + Mul<Output = T>> std::ops::Mul<T> for Matrix<T> {
    type Output = Self;

    fn mul(mut self, scalar: T) -> Self::Output {
        self.multiply_by_scalar(scalar);
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let mat: Matrix<u8> = Matrix::new(3,3);
        assert_eq!(format!("{}",mat),"0 0 0 \n0 0 0 \n0 0 0 \n");
    }
}
