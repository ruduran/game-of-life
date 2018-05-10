use std;
use std::ops::{Index,IndexMut};

pub struct Matrix<T> {
    data: Vec<T>,
    pub width: usize,
    pub height: usize
}


impl<T> Matrix<T> where T: std::clone::Clone {
    pub fn new(width: usize, height: usize, default: T) -> Matrix<T> {
        Matrix {
            data: vec![default; width*height],
            width,
            height
        }
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, _index: usize) -> &[T] {
        &self.data[_index*self.width..(_index+1)*self.width]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, _index: usize) -> &mut [T] {
        &mut self.data[_index*self.width..(_index+1)*self.width]
    }
}