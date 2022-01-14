use crate::piece::{print_solution, Piece};
use crate::placement::get_placements;
use std::sync::{Arc, Mutex};
use std::thread;
use rayon::prelude::*;

// pub fn solve<'a, I>(open_square_labels_opt: Option<I>)
// where
//     I: IntoIterator<Item = &'a str>,
// {
//     let mut starting_board = Piece::starting_board();
//     if let Some(open_square_labels) = open_square_labels_opt {
//         for open_square_label in open_square_labels {
//             starting_board.mark_coord_for(open_square_label)
//         }
//     }
//     println!("{}", starting_board);

//     let placements: Vec<Vec<Piece>> = Piece::playing_pieces()
//         .into_iter()
//         .map(|piece| get_placements(&piece))
//         .collect();

//     let mut count = 0;

//     let mut solution_callback = |solution_and_board_| {
//         count += 1;
//         println!("found a solution #{}", count);
//     };

//     _solve(
//         Vec::new(),
//         starting_board,
//         placements,
//         &mut solution_callback,
//     );

//     // println!("{:?}", placements);
// }

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

    let piece_placements = Arc::new(build_piece_placements());

    let mut partial_solutions = Vec::new();
    let mut partial_callback = |pieces, board| {
        partial_solutions.push((pieces, board));
    };
    let partial_piece_placements = Arc::clone(&piece_placements);
    // let partial_piece_placements = build_piece_placements();

    _solve(
        Vec::new(),
        starting_board,
        &partial_piece_placements,
        &mut partial_callback,
        Some(1),
    );

    let mut threads = Vec::new();
    let solutions_found_mutex = Arc::new(Mutex::new(0));

    for (partial_pieces, partial_board) in partial_solutions {
        let thread_piece_placements = Arc::clone(&piece_placements);
        let thread_solutions_found_mutex = Arc::clone(&solutions_found_mutex);
        let mut callback = move |pieces, _| {
            *thread_solutions_found_mutex.lock().unwrap() += 1;
            println!("{}", *thread_solutions_found_mutex.lock().unwrap());
            print_solution(pieces);
        };

        threads.push(thread::spawn(move || {
            _solve(
                partial_pieces,
                partial_board,
                &thread_piece_placements,
                &mut callback,
                None,
            );
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }
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
