use crate::piece::Piece;

pub fn solve<'a, I>(open_square_labels: Option<I>)
where
    I: IntoIterator<Item = &'a str>,
{
    let mut starting_board = Piece::starting_board();
    if let Some(open_square_vals) = open_square_labels {
        for open_square_val in open_square_vals {
            if let Some(&(row, col)) = Piece::coord_for(open_square_val) {
                starting_board[row][col] = 1;
            }
        }
    }
    println!("{}", starting_board);

    let placements: Vec<Vec<Piece>> = Piece::playing_pieces()
        .into_iter()
        .map(|piece| piece.get_placements())
        .collect();

    let mut count = 0;

    let solution_callback = &mut |solution: &Vec<&Piece>| {
        count += 1;
        println!("found a solution #{}", count);
    };

    _solve(Vec::new(), starting_board, &placements, solution_callback)
}

fn _solve<'a, F>(
    cur_board_pieces: Vec<&'a Piece>,
    cur_board: Piece,
    piece_placements: &'a Vec<Vec<Piece>>,
    solution_callback: &mut F,
) where
    F: FnMut(&Vec<&Piece>) -> (),
{
    if cur_board_pieces.len() == piece_placements.len() {
        solution_callback(&cur_board_pieces);
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
