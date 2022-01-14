use crate::piece::{print_solution, Piece};
use crate::placement::get_placements;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

fn build_starting_board_with_opens<'a>(
    open_square_labels_opt: Option<impl IntoIterator<Item = &'a str>>,
) -> Piece {
    let mut starting_board = Piece::starting_board();
    if let Some(open_square_labels) = open_square_labels_opt {
        for open_square_label in open_square_labels {
            starting_board.mark_coord_for(open_square_label)
        }
    }
    starting_board
}

fn build_piece_placements() -> Vec<Vec<Piece>> {
    Piece::playing_pieces()
        .into_iter()
        .map(|piece| get_placements(&piece))
        .collect()
}

pub fn solve_threaded<'a>(open_square_labels_opt: Option<impl IntoIterator<Item = &'a str>>) {
    let starting_board = build_starting_board_with_opens(open_square_labels_opt);

    let piece_placements = build_piece_placements();

    let mut partial_solutions = Vec::new();
    let mut partial_callback = |pieces, board| {
        partial_solutions.push((pieces, board));
    };

    // first do a partial solve, placing the first piece in all its placements
    // then we will have partial solutions to iterate over with threads/rayon
    _solve(
        Vec::new(),
        starting_board,
        &piece_placements,
        &mut partial_callback,
        Some(1),
    );

    let solutions_found_mutex = Arc::new(Mutex::new(0));

    partial_solutions
        .par_iter()
        .map(|(partial_pieces, partial_board)| {
            let thread_solutions_found_mutex = Arc::clone(&solutions_found_mutex);
            let mut callback = move |pieces: Vec<Piece>, _| {
                let mut solutions_found = thread_solutions_found_mutex.lock().unwrap();
                *solutions_found += 1;
                println!("{}", *solutions_found);
                print_solution(&pieces);
            };

            _solve(
                partial_pieces.to_vec(),
                *partial_board,
                &piece_placements,
                &mut callback,
                None,
            )
        })
        .for_each(|()| {}); // rayon won't start the work until there's something that consumes it.
}

fn _solve(
    cur_board_pieces: Vec<Piece>,
    cur_board: Piece,
    piece_placements: &[Vec<Piece>],
    solution_callback: &mut impl FnMut(Vec<Piece>, Piece) -> (),
    stop_at: Option<usize>,
) {
    if cur_board_pieces.len() == stop_at.unwrap_or(piece_placements.len()) {
        solution_callback(cur_board_pieces, cur_board);
    } else {
        for placement in &piece_placements[cur_board_pieces.len()] {
            let new_board = cur_board + *placement;
            if new_board.is_flat() {
                let mut new_board_pieces = cur_board_pieces.clone();
                new_board_pieces.push(*placement);
                _solve(
                    new_board_pieces,
                    new_board,
                    piece_placements,
                    solution_callback,
                    stop_at,
                );
            }
        }
    }
}
