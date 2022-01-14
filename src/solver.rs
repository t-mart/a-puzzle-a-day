use crate::piece::{Piece, print_solution};
use crate::placement::get_placements;
use std::sync::{Arc, Mutex};
use std::thread;

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

    let piece_placements = build_piece_placements();

    let mut partial_solutions = Vec::new();
    let mut partial_callback = |pieces, board| {
        partial_solutions.push((pieces, board));
    };

    _solve(
        Vec::new(),
        starting_board,
        &piece_placements,
        &mut partial_callback,
        Some(1)
    );

    let mut solution_count = 0;
    let mut callback = |pieces, _| {
        solution_count += 1;
        println!("found solution #{}", solution_count);
        print_solution(pieces);
    };

    for (partial_pieces, partial_board) in partial_solutions {
        _solve(
            partial_pieces,
            partial_board,
            &piece_placements,
            &mut callback,
            None
        );
    }
}

fn _solve<'a>(
    cur_board_pieces: Vec<&'a Piece>,
    cur_board: Piece,
    piece_placements: &'a [Vec<Piece>],
    solution_callback: &mut impl FnMut(Vec<&'a Piece>, Piece) -> (),
    stop_at: Option<usize>
) {
    if cur_board_pieces.len() == stop_at.unwrap_or(piece_placements.len()) {
        solution_callback(cur_board_pieces, cur_board);
    } else {
        for placement in &piece_placements[cur_board_pieces.len()] {
            let new_board = cur_board + *placement;
            if new_board.is_flat() {
                let mut new_board_pieces = cur_board_pieces.clone();
                new_board_pieces.push(&placement);
                _solve(
                    new_board_pieces,
                    new_board,
                    piece_placements,
                    solution_callback,
                    stop_at
                );
            }
        }
    }
}
