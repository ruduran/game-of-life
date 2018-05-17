use std;
use std::ops::{Index,IndexMut};

#[derive(Clone)]
pub struct Row<T> {
    data: Vec<T>,
    length: usize,
    default: T
}

impl<T> Row<T> where T: std::clone::Clone {
    pub fn new(length: usize, default: T) -> Row<T> {
        Row {
            data: vec![default.clone(); length],
            length,
            default
        }
    }

    pub fn valid_index(&self, i: usize) -> bool {
        return i < self.length
    }
}

impl<T> Index<usize> for Row<T> where T: std::clone::Clone {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        if self.valid_index(i) {
            &self.data[i]
        } else {
            &self.default
        }
    }
}

impl<T> IndexMut<usize> for Row<T> where T: std::clone::Clone {
    fn index_mut(&mut self, i: usize) -> &mut T {
        if self.valid_index(i) {
            &mut self.data[i]
        } else {
            &mut self.default
        }
    }
}


pub struct Matrix<T> {
    data: Vec<Row<T>>,
    pub width: usize,
    pub height: usize,
    empty_row: Row<T>
}

impl<T> Matrix<T> where T: std::clone::Clone {
    pub fn new(width: usize, height: usize, default: T) -> Matrix<T> {
        Matrix {
            data: vec![Row::new(width, default.clone()); height],
            width,
            height,
            empty_row: Row::new(0, default)
        }
    }

    pub fn valid_index(&self, i: usize) -> bool {
        return i < self.height
    }
}

impl<T> Index<usize> for Matrix<T> where T: std::clone::Clone  {
    type Output = Row<T>;

    fn index(&self, i: usize) -> &Row<T> {
        if self.valid_index(i) {
            &self.data[i]
        } else {
            &self.empty_row
        }
    }
}

impl<T> IndexMut<usize> for Matrix<T> where T: std::clone::Clone  {
    fn index_mut(&mut self, i: usize) -> &mut Row<T> {
        if self.valid_index(i) {
            &mut self.data[i]
        } else {
            &mut self.empty_row
        }
    }
}
