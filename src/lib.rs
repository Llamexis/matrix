use std::fmt;
use std::ops::{Index, Mul, IndexMut};


#[derive(Debug)]
pub struct Matrix<T: Default + Clone >{
    cols: usize,
    rows: usize,
    data: Box<Vec<T>>,
}

impl<T: Default + Clone > Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Matrix<T> {
    let  data = Box::new(vec![T::default();cols*rows]);
        Self{
            cols,
            rows,
            data,
        }
    }
    pub fn from_data(rows:usize, cols: usize, data: &[T]) -> Self {
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

impl<T: Default + Clone> Index<(usize, std::ops::Range<usize>)> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: (usize, std::ops::Range<usize>)) -> &Self::Output {
        let (r, cols) = index;
        let start_idx = r * self.cols + cols.start;
        let end_idx = r * self.cols + cols.end;
        &self.data[start_idx..end_idx]
    }
}
impl<T: Default + Clone> Index<(usize, std::ops::RangeFull)> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: (usize, std::ops::RangeFull)) -> &Self::Output {
        let (r, _cols) = index;
        let start_idx = r * self.cols;
        let end_idx = r * self.cols + self.cols;
        &self.data[start_idx..end_idx]
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
    const DATA: [i32; 9] = [1,2,3,4,5,6,7,8,9];

    #[test]
    fn new() {
        let mat: Matrix<u8> = Matrix::new(3,3);
        assert_eq!(format!("{}",mat),"0 0 0 \n0 0 0 \n0 0 0 \n");
    }

    #[test]
    fn from_data() {
        let mat = Matrix::from_data(3,3,&[1.1,2.1,3.1,4.1,5.1,6.1,7.1,8.1,9.1]);
        assert_eq!(format!("{}",mat),"1.1 2.1 3.1 \n4.1 5.1 6.1 \n7.1 8.1 9.1 \n");
    }

    #[test]
    #[ignore = "skipped"]
    fn transpose() {
        
    }

    #[test]
    fn simple_indexing() {
        let mut mat: Matrix<u8> = Matrix::new(3,4);
        mat[(0, 0)] = 1;
        mat[(0, 1)] = 2;
        mat[(0, 2)] = 3;
        mat[(0, 3)] = 4;

        mat[(1, 0)] = 5;
        mat[(1, 1)] = 6;
        mat[(1, 2)] = 7;
        mat[(1, 3)] = 8;

        mat[(2, 0)] = 9;
        mat[(2, 1)] = 10;
        mat[(2, 2)] = 11;
        mat[(2, 3)] = 12;

        assert_eq!(format!("{}",mat),"1 2 3 4 \n5 6 7 8 \n9 10 11 12 \n");
    } 

    #[test]
    fn row_indexing(){
        let mut mat: Matrix<u8> = Matrix::new(3,4);
        mat[(0, 0)] = 1;
        mat[(0, 1)] = 2;
        mat[(0, 2)] = 3;
        mat[(0, 3)] = 4;

        mat[(1, 0)] = 5;
        mat[(1, 1)] = 6;
        mat[(1, 2)] = 7;
        mat[(1, 3)] = 8;

        mat[(2, 0)] = 9;
        mat[(2, 1)] = 10;
        mat[(2, 2)] = 11;
        mat[(2, 3)] = 12;
        
        let row = &mat[(0,..)];
        assert_eq!(format!("{:?}",row),format!("{:?}",&[1,2,3,4]));
        let row = &mat[(0,1..4)];
        assert_eq!(format!("{:?}",row),format!("{:?}",&[2,3,4]));
        mat = mat.transpose();
        let row = &mat[(0,..)];
        assert_eq!(format!("{:?}",row),format!("{:?}",&[1,5,9]));
    }

    #[test]
    fn multiply_by_scalar() {
        let scalar = 3;
        let mat = Matrix::from_data(3,3,&DATA);
        let mut multipied_data = Vec::new();
        for i in DATA {
            multipied_data.push(i * scalar);
        }
        let expected_mat = Matrix::from_data(3,3,multipied_data.as_slice());
        assert_eq!(format!("{:?}",mat*scalar),format!("{:?}",expected_mat));
    }
}
