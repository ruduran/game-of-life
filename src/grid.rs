use std::ops::{Index,IndexMut};

pub struct Grid {
    grid: Vec<bool>,
    pub width: usize,
    pub height: usize
}


impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            grid: vec![false; width * height],
            width,
            height
        }
    }
}

impl Index<usize> for Grid {
    type Output = [bool];

    fn index(&self, _index: usize) -> &[bool] {
        &self.grid[_index*self.width..(_index+1)*self.width]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, _index: usize) -> &mut [bool] {
        &mut self.grid[_index*self.width..(_index+1)*self.width]
    }
}
