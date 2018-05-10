extern crate ncurses;

use ncurses::*;

mod grid;
use grid::Grid;

// TODO: Improve and refactor
fn neighbours(g: & Grid, h: usize, w: usize) -> usize {
    let mut count = 0;

    if h > 0 && w > 0 && g[h-1][w-1] {
        count += 1;
    }
    if h > 0 && g[h-1][w] {
        count += 1;
    }
    if h > 0 && w < g.width-1 && g[h-1][w+1] {
        count += 1;
    }
    if w < g.width-1 && g[h][w+1] {
        count += 1;
    }
    if h < g.height-1 && w < g.width-1 && g[h+1][w+1] {
        count += 1;
    }
    if h < g.height-1 && g[h+1][w] {
        count += 1;
    }
    if h < g.height-1 && w > 0 && g[h+1][w-1] {
        count += 1;
    }
    if w > 0 && g[h][w-1] {
        count += 1;
    }

    count
}

fn iterate(g: &Grid) -> Grid {
    let mut next_grid = Grid::new(g.width, g.height);
    for h in 0..g.height {
        for w in 0..g.width {
            let n = neighbours(g, h, w);
            if g[h][w] {
                next_grid[h][w] = n >= 2 && n <= 3;
            } else {
                next_grid[h][w] = n == 3;
            }
        }
    }
    next_grid
}

fn initialize() {
    let locale_conf = LcCategory::all;
    setlocale(locale_conf, "en_US.UTF-8");

    initscr();
}

fn print_grid(g: &Grid) {
    clear();

    for h in 0..g.height {
        for w in 0..g.width {
            if g[h][w] {
                printw("X");
            } else {
                printw(" ");
            }
        }
        printw("\n");
    }
    refresh();
}

fn main() {
    initialize();

    let mut g = Grid::new(8, 10);

    g[0][1] = true;
    g[1][2] = true;
    g[2][0] = true;
    g[2][1] = true;
    g[2][2] = true;

    let finishing_char = 'q' as i32;
    print_grid(&g);
    while getch() != finishing_char {
        g = iterate(&g);
        print_grid(&g);
    }

    endwin();
}
