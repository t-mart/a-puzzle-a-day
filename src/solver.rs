use crate::piece::Piece;
use crate::placement::get_placements;

pub fn solve<'a, I>(open_square_labels_opt: Option<I>)
where
    I: IntoIterator<Item = &'a str>,
{
    let mut starting_board = Piece::starting_board();
    if let Some(open_square_labels) = open_square_labels_opt {
        for open_square_label in open_square_labels {
            starting_board.mark_coord_for(open_square_label)
        }
    }
    println!("{}", starting_board);

    let placements: Vec<Vec<Piece>> = Piece::playing_pieces()
        .into_iter()
        .map(|piece| get_placements(&piece))
        .collect();

    let mut count = 0;

    let mut solution_callback = |_solution| {
        count += 1;
        println!("found a solution #{}", count);
    };

    _solve(Vec::new(), starting_board, &placements, &mut solution_callback)
}

pub fn solve_threaded<'a, I>(open_square_labels_opt: Option<I>)
where
    I: IntoIterator<Item = &'a str>,
{
    let mut starting_board = Piece::starting_board();
    if let Some(open_square_labels) = open_square_labels_opt {
        for open_square_label in open_square_labels {
            starting_board.mark_coord_for(open_square_label)
        }
    }
    println!("{}", starting_board);

    let mut first: Vec<_> = Piece::playing_pieces()
        .into_iter()
        .map(|piece| get_placements(&piece))
        .collect();
    // println!("first len: {}", first.len());
    let rest = first.split_off(1);
    // println!("first len: {}, rest len: {}", first.len(), rest.len());

    let mut partial_solutions = Vec::new();
    let mut solution_callback = |solution| {
        println!("found solution");
        for piece in &solution {
            println!("{}", piece);
        }
        partial_solutions.push(solution);
    };

    _solve(Vec::new(), starting_board, &first, &mut solution_callback);
}

fn _solve<'a>(
    cur_board_pieces: Vec<&'a Piece>,
    cur_board: Piece,
    piece_placements: &'a Vec<Vec<Piece>>,
    solution_callback: &mut impl FnMut(Vec<&'a Piece>) -> (),
) {
    if cur_board_pieces.len() == piece_placements.len() {
        solution_callback(cur_board_pieces);
    } else {
        for placement in &piece_placements[cur_board_pieces.len()] {
            let new_board = cur_board + *placement;
            if new_board.is_flat() {
                let mut new_board_pieces = cur_board_pieces.clone();
                new_board_pieces.push(placement);
                _solve(
                    new_board_pieces,
                    new_board,
                    piece_placements,
                    solution_callback,
                );
            }
        }
    }
}
