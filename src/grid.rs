use ncurses::*;

use matrix::Matrix;

pub struct Grid {
    matrix: Matrix<bool>
}

impl Grid {
    // TODO: Simplify once an initial grid can be loaded/given
    pub fn new(width: usize, height: usize) -> Grid {
        let mut matrix = Matrix::<bool>::new(width, height, false);
        matrix[0][1] = true;
        matrix[1][2] = true;
        matrix[2][0] = true;
        matrix[2][1] = true;
        matrix[2][2] = true;
        Grid {
            matrix
        }
    }

    // TODO: Abstract ncurses usage
    pub fn print(&self) {
        clear();

        for h in 0..self.matrix.height {
            for w in 0..self.matrix.width {
                if self.matrix[h][w] {
                    printw("X");
                } else {
                    printw(" ");
                }
            }
            printw("\n");
        }
        refresh();
    }

    // TODO: Improve
    fn neighbours(&self, h: usize, w: usize) -> usize {
        let mut count = 0;

        if h > 0 && w > 0 && self.matrix[h-1][w-1] {
            count += 1;
        }
        if h > 0 && self.matrix[h-1][w] {
            count += 1;
        }
        if h > 0 && w < self.matrix.width-1 && self.matrix[h-1][w+1] {
            count += 1;
        }
        if w < self.matrix.width-1 && self.matrix[h][w+1] {
            count += 1;
        }
        if h < self.matrix.height-1 && w < self.matrix.width-1 && self.matrix[h+1][w+1] {
            count += 1;
        }
        if h < self.matrix.height-1 && self.matrix[h+1][w] {
            count += 1;
        }
        if h < self.matrix.height-1 && w > 0 && self.matrix[h+1][w-1] {
            count += 1;
        }
        if w > 0 && self.matrix[h][w-1] {
            count += 1;
        }

        count
    }

    pub fn iterate(&mut self) {
        let mut next_matrix = Matrix::<bool>::new(self.matrix.width, self.matrix.height, false);
        for h in 0..self.matrix.height {
            for w in 0..self.matrix.width {
                let n = self.neighbours(h, w);
                if self.matrix[h][w] {
                    next_matrix[h][w] = n >= 2 && n <= 3;
                } else {
                    next_matrix[h][w] = n == 3;
                }
            }
        }
        self.matrix = next_matrix;
    }
}
