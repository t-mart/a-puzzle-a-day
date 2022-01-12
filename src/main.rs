#![feature(array_zip)]
#![feature(once_cell)]

mod piece;
mod solver;

use clap::{App, Arg};
use piece::BOARD_LABELS;
use solver::solve;

fn main() {
    let matches = App::new("A-Puzzle-A-Day Solver")
        .about("Generates solutions for A-Puzzle-A-Day, a wooden combination puzzle")
        .arg(
            Arg::new("OPEN_SQUARE")
                .takes_value(true)
                .max_values(2)
                .possible_values(BOARD_LABELS.keys().into_iter().collect::<Vec<&&str>>())
                .help(
                    "The label of a square of the board to fix open in generated \
                    solutions. 0, 1 or 2 may be given. Each square not given will \
                    be considered a wildcard. Examples are \"apr\" or \"23\". To \
                    find all solutions, do not give any values to this argument.",
                ),
        )
        .get_matches();

    solve(matches.values_of("OPEN_SQUARE"));
}
