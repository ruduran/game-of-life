extern crate argparse;
extern crate ncurses;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use ncurses::*;
use argparse::{ArgumentParser, Store};

mod grid;
mod matrix;
use grid::Grid;

fn initialize() {
    let locale_conf = LcCategory::all;
    setlocale(locale_conf, "en_US.UTF-8");

    initscr();
}

struct Options {
    filename: String,
}

fn parse_args() -> Options {
    let mut options = Options {filename: String::from("")};
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Game Of Life.");
        ap.refer(&mut options.filename)
            .required()
            .add_option(&["-f", "--filename"], Store,
                        "TODO");
        ap.parse_args_or_exit();
    }
    options
}

fn main() {
    let options = parse_args();


    initialize();

    let mut g = Grid::new(0, 0);
    g.load(&options.filename);
    g.print();

    let mut c = getch();
    while c != 'q' as i32 {
        if c == 's' as i32 {
            g.save(&options.filename);
        } else {
            g.iterate();
        }
        g.print();
        c = getch();
    }

    endwin();
}
