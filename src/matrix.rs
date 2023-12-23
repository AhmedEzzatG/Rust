use std::ops::{Add, Index, IndexMut, Mul};
use matrix_row::MatrixRow;


mod matrix_row;

#[derive(Debug)]
pub struct Matrix<T> {
    num_of_rows: usize,
    num_of_cols: usize,
    rows: Vec<MatrixRow<T>>,
}

impl<T: Clone> Matrix<T> {
    pub fn new(num_of_rows: usize, num_of_cols: usize, default_value: T) -> Matrix<T> {
        Matrix {
            num_of_rows,
            num_of_cols,
            rows: vec![MatrixRow::new(num_of_cols, default_value); num_of_rows]
        }
    }
    pub fn identity(n : usize, value: T, default : T) -> Matrix<T> {
        let mut res = Matrix::new(n, n, default);
        for i in 0..n {
            res[i][i] = value.clone();
        }
        res
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = MatrixRow<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}

impl<T: Add<Output = T> + Clone + Default> Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.num_of_rows != rhs.num_of_rows || self.num_of_cols != rhs.num_of_cols {
            panic!("this matrices is not identical size");
        }
        let mut result = Matrix::new(self.num_of_rows, self.num_of_cols, T::default());
        for i in 0..self.num_of_rows {
            for j in 0..self.num_of_cols {
               result[i][j] = self[i][j].clone() + rhs[i][j].clone();
            }
        }
        result
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Clone + Default> Mul for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.num_of_cols != rhs.num_of_rows {
            panic!("number of columns in lhs matrix not equal to number of rows in rhs matrix");
        }
        let mut result = Matrix::new(self.num_of_rows, self.num_of_cols, T::default());
        for i in 0..self.num_of_rows {
            for j in 0..self.num_of_cols {
                for k in 0..rhs.num_of_cols {
                    result[i][k] = result[i][k].clone() + (self[i][j].clone() * rhs[j][k].clone());
                }
            }
        }
        result
    }
}
