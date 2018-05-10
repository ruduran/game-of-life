extern crate ncurses;

use ncurses::*;

mod grid;
mod matrix;
use grid::Grid;

fn initialize() {
    let locale_conf = LcCategory::all;
    setlocale(locale_conf, "en_US.UTF-8");

    initscr();
}

fn main() {
    initialize();

    let mut g = Grid::new(8, 10);

    let finishing_char = 'q' as i32;
    g.print();
    while getch() != finishing_char {
        g.iterate();
        g.print();
    }

    endwin();
}
