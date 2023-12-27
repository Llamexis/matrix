use std::fmt;
use std::ops::{Index, Mul, IndexMut, Add};

#[derive(Debug)]
pub struct Matrix<T: Default + Clone >{
    pub cols: usize,
    pub rows: usize,
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
    pub fn iter(&self) -> MatrixIterator<'_, T> {

        MatrixIterator { iterator: 0, data: &self.data }
    }
    pub fn iter_mut(&mut self) -> MatrixMutIterator<'_, T> {
        MatrixMutIterator {iterator: 0, data :&mut self.data}
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
impl<T: Default + Clone + Copy + Mul<Output = T> + Add<Output = T>> Mul for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, mat: Matrix<T>) -> Self {
        assert!(self.cols == mat.rows,"Wrong dimensions. Try transposing one of matrix.\nDims: mat1 - {:?} != mat2 - {:?}", self.dim(), mat.dim());
        let mut out: Matrix<T> = Matrix::new(self.rows, mat.cols);
        for i in 0..self.rows {
            for j in 0..mat.cols {
                for k in 0..mat.rows {
                    let tmp = self[(i,k)] * mat[(k,j)];
                    out[(i,j)] = out[(i,j)] + tmp;
                }
            }
        }
        out
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

impl<T:Default + Clone > Index<(usize,&usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, (row,col): (usize,&usize)) -> &Self::Output {
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

impl<T:Default + Clone > IndexMut<(usize,&usize)> for Matrix<T> {
    fn index_mut(&mut self, (row,col): (usize,&usize)) -> &mut T {
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

impl<T: Default + Clone> std::iter::Iterator for MatrixIterator<'_, T>{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator {
            i if i < self.data.len() => {
                let tmp = &self.data[i];
                self.iterator=self.iterator + 1;
                Some(tmp.clone())
            }
            _ => None,
        }
    }

}

impl <'a, T: Default + Clone> std::iter::Iterator for MatrixMutIterator<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {

        match self.iterator {
            i if i < self.data.len() => {
                let ptr = self.data.as_mut_ptr();
                self.iterator += 1;
                unsafe {
                    Some(&mut *ptr.add(i))
                }
            }
            _ => None,
        }
    }
    
}
pub struct MatrixIterator<'a, T> {
    iterator: usize,
    data: &'a [T],
}
pub struct MatrixMutIterator<'a, T> {
    iterator: usize,
    data: &'a mut [T],
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: [i32; 9] = [1,2,3,4,5,6,7,8,9];

    #[test]
    fn iteration() {
        let mat: Matrix<i32> = Matrix::from_data(3,3,&DATA);
        let mut mat = mat.iter();
        let mut tmp = DATA.iter();
        assert_eq!(tmp.next(),mat.next().as_ref());
        assert_eq!(tmp.next(),mat.next().as_ref());
        assert_eq!(tmp.next(),mat.next().as_ref());
        assert_eq!(tmp.next(),mat.next().as_ref());
        assert_eq!(tmp.next(),mat.next().as_ref());
        assert_eq!(tmp.next(),mat.next().as_ref());
        assert_eq!(tmp.next(),mat.next().as_ref());
        assert_eq!(tmp.next(),mat.next().as_ref());
        assert_eq!(tmp.next(),mat.next().as_ref());
        assert_eq!(None, mat.next().as_ref());
    }

    #[test]
    fn mut_iteration() {
        let mut mat: Matrix<i32> = Matrix::from_data(3,3,&DATA);
        let mut mat = mat.iter_mut();
        let mut tmp = DATA.iter();
        assert_eq!(tmp.next(),mat.next().as_deref());
        assert_eq!(tmp.next(),mat.next().as_deref());
        assert_eq!(tmp.next(),mat.next().as_deref());
        assert_eq!(tmp.next(),mat.next().as_deref());
        assert_eq!(tmp.next(),mat.next().as_deref());
        assert_eq!(tmp.next(),mat.next().as_deref());
        assert_eq!(tmp.next(),mat.next().as_deref());
        assert_eq!(tmp.next(),mat.next().as_deref());
        assert_eq!(tmp.next(),mat.next().as_deref());
        assert_eq!(None, mat.next().as_ref());
    }
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
    fn transpose() {
       let mat = Matrix::from_data(3,3,&DATA); 
       let expected = Matrix::from_data(3,3,&[1,4,7,2,5,8,3,6,9]);
       assert_eq!(format!("{}",mat.transpose()),format!("{}",expected));
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

    #[test]
    fn test_multiply() {
        let mat1: Matrix<i32> = Matrix::from_data(2, 3, &[1, 2, 3, 4, 5, 6]);
        let mat2: Matrix<i32> = Matrix::from_data(3, 2, &[7, 8, 9, 10, 11, 12]);
        let expected_result: Matrix<i32> = Matrix::from_data(2, 2, &[58, 64, 139, 154]);

        println!("M1\n{}",mat1);
        println!("M2\n{}",mat2);
        let result = mat1 * mat2;
        assert_eq!(result.dim(), expected_result.dim());
        println!("MR\n{}",result);
        for i in 0..result.rows {
            for j in 0..result.cols {
                assert_eq!(result[(i, j)], expected_result[(i, j)]);
            }
        }
    }
    #[test]
    #[should_panic]
    fn test_multiply_wrong_dimensions() {
        let mat1: Matrix<i32> = Matrix::new(2, 3);
        let mat2: Matrix<i32> = Matrix::new(4, 2);

        let _result = mat1 * mat2;
    }
    #[test]
    fn test_multiply_by_vector() {
        let mat = Matrix::from_data(3,3,&DATA);
        let vec = Matrix::from_data(1,3,&[1,2,3]);
        let res = vec * mat;
        assert_eq!(format!("{}",res), "30 36 42 \n");

        let vec = Matrix::from_data(1,3,&[1,2,3]);
        let mat = Matrix::from_data(3,3,&DATA);
        let res = mat*vec.transpose();
        assert_eq!(format!("{}",res), "14 \n32 \n50 \n");
    }

}
