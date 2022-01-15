use crate::piece::{Piece, Solution};
use crate::placement::get_placements;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub fn solve<'a>(labels_opt: Option<impl Iterator<Item = &'a str>>) {
    let mut starting_board = Piece::starting_board();
    match labels_opt {
        Some(labels) => {
            let mut label_str_vec = vec![];
            for label in labels {
                starting_board.mark_coord_for(label);
                label_str_vec.push(format!("\"{}\"", label));
            }
            println!(
                "Solving for solutions with {} exposed...\n",
                label_str_vec.join(" and ")
            )
        }
        None => println!("Solving for all solutions...\n"),
    }

    let piece_placements = Piece::playing_pieces()
        .into_iter()
        .map(|piece| get_placements(&piece))
        .collect::<Vec<_>>();
    let piece_placement_slices = &piece_placements
        .iter()
        .map(|pieces| pieces.as_slice())
        .collect::<Vec<_>>();

    let mut partial_solutions = Vec::new();
    let mut partial_callback = |pieces: &[Piece], board| {
        partial_solutions.push((pieces.to_owned(), board));
    };

    // first do a partial solve, placing the first piece in all its placements
    // then we will have partial solutions to iterate over with threads/rayon
    _solve(
        Vec::new(),
        starting_board,
        piece_placement_slices,
        &mut partial_callback,
        Some(1),
    );

    let solutions_found_mutex = Arc::new(Mutex::new(0));

    partial_solutions
        .par_iter()
        .map(|(partial_pieces, partial_board)| {
            let thread_solutions_found_mutex = Arc::clone(&solutions_found_mutex);
            let mut callback = move |pieces: &[Piece], _| {
                let mut solutions_found = thread_solutions_found_mutex.lock().unwrap();
                *solutions_found += 1;
                let solution = Solution(pieces);
                println!("#{}\n{}", *solutions_found, solution);
            };

            _solve(
                partial_pieces.to_vec(),
                *partial_board,
                piece_placement_slices,
                &mut callback,
                None,
            )
        })
        .for_each(drop); // exhaust the iterator so that the work is actually dont

    println!("{} solutions found", solutions_found_mutex.lock().unwrap());
}

fn _solve(
    cur_board_pieces: Vec<Piece>,
    cur_board: Piece,
    piece_placements: &[&[Piece]],
    solution_callback: &mut impl FnMut(&[Piece], Piece),
    stop_at: Option<usize>,
) {
    if cur_board_pieces.len() == stop_at.unwrap_or(piece_placements.len()) {
        solution_callback(&cur_board_pieces, cur_board);
    } else {
        for placement in piece_placements[cur_board_pieces.len()] {
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
