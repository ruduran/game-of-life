use serde_json;
use ncurses::*;

use std::fs::File;

use matrix::Matrix;

pub struct Grid {
    matrix: Matrix<bool>
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            matrix: Matrix::<bool>::new(width, height, false)
        }
    }

    // TODO: Abstract ncurses usage
    pub fn print(&self) {
        clear();

        for h in 0..self.matrix.height {
            for w in 0..self.matrix.width {
                if self.matrix[h][w] {
                    printw("█");
                } else {
                    printw(" ");
                }
            }
            printw("\n");
        }
        refresh();
    }

    fn neighbours(&self, h: usize, w: usize) -> usize {
        let mut count = 0;

        for c in 0..3 {
            for r in 0..3 {
                if !(c == 1 && r == 1) &&
                   h + c > 0 && w + r > 0 &&
                   self.matrix[h + c - 1][w + r - 1] {
                    count += 1;
                }
            }
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

    pub fn load (&mut self, path: &String) {
        let file = File::open(path).unwrap();
        let g: SerializableGrid = serde_json::from_reader(file).unwrap();

        let mut new_matrix = Matrix::<bool>::new(g.width, g.height, false);
        for cell in g.elements.iter() {
            new_matrix[cell.height][cell.width] = cell.alive;
        }
        self.matrix = new_matrix;
    }

    pub fn save(&self, path: &String) {
        let mut g = SerializableGrid {
            height: self.matrix.height,
            width: self.matrix.width,
            elements: Vec::<Cell>::new()
        };

        for h in 0..self.matrix.height {
            for w in 0..self.matrix.width {
                if self.matrix[h][w] {
                    let c = Cell {
                        height: h,
                        width: w,
                        alive: self.matrix[h][w]
                    };
                    g.elements.push(c);
                }
            }
        }


        let file = File::create(path).unwrap();
        serde_json::to_writer_pretty(file, &g);
    }
}


#[derive(Serialize, Deserialize)]
struct Cell {
    height: usize,
    width: usize,
    alive: bool
}

#[derive(Serialize, Deserialize)]
struct SerializableGrid {
    height: usize,
    width: usize,
    elements: Vec<Cell>,
}
