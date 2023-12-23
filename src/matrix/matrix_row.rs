use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct MatrixRow<T> {
    values: Vec<T>,
}

impl<T> Index<usize> for MatrixRow<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<T> IndexMut<usize> for MatrixRow<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl<T : Clone> MatrixRow<T> {
    pub fn new(n: usize, default: T) -> MatrixRow<T> {
        MatrixRow {
            values: vec![default; n],
        }
    }
}

