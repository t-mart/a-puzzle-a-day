#![feature(array_zip)]

mod piece;
mod placement;
mod solver;

use clap::{App, Arg};
use piece::Piece;
use solver::solve;

const OPEN_LABEL: &str = "OPEN_LABEL";

fn main() {
    let matches = App::new("A-Puzzle-A-Day Solver")
        .about(
            "Generates solutions for A-Puzzle-A-Day, a combination puzzle \
            with the goal of arranging 8 tiles on a board such that a particular \
            date (2 squares) on the board is left exposed.",
        )
        .arg(
            Arg::new(OPEN_LABEL)
                .takes_value(true)
                .max_values(2)
                .possible_values(Piece::get_labels())
                .help(
                    "The label of a square of the board to fix open in generated \
                    solutions. Up to 2 labels may be given. Each label not given will \
                    be considered a wildcard. (To find all solutions, do not provide \
                    this argument.)",
                ),
        )
        .get_matches();

    solve(matches.values_of(OPEN_LABEL));
}
