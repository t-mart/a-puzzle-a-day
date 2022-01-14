use crate::piece::{Piece, PIECE_SIZE};

use std::collections::HashSet;

/// Return a Vec of all Pieces that occur if the piece was only rotated and/or flipped.
/// The piece is shoved up and to the left each time. No duplicates are in the Vec.
pub fn get_orientations(piece: &Piece) -> Vec<Piece> {
    let mut orientations = HashSet::new();
    let mut tmp = *piece;

    orientations.insert(tmp);

    for _ in 0..3 {
        tmp.rotate90();
        tmp.shove_left_up();
        orientations.insert(tmp);
    }

    tmp.flip_updown();
    tmp.shove_left_up();
    orientations.insert(tmp);

    for _ in 0..3 {
        tmp.rotate90();
        tmp.shove_left_up();
        orientations.insert(tmp);
    }

    orientations.into_iter().collect()
}

/// Return a Vec of all Pieces that could occur if the piece was shifted around the board
/// without rotations or flips
pub fn get_shifts(piece: &Piece) -> Vec<Piece> {
    let mut placements = Vec::new();
    let mut first_clear_col = PIECE_SIZE - 1;
    let mut first_clear_row = PIECE_SIZE - 1;
    let mut tmp_row_offsetted = *piece;

    while piece.col(first_clear_col).iter().sum::<u8>() == 0 {
        first_clear_col -= 1;
    }

    while piece[first_clear_row].iter().sum::<u8>() == 0 {
        first_clear_row -= 1;
    }

    for _ in 0..(PIECE_SIZE - first_clear_row) {
        let mut tmp_col_offsetted = tmp_row_offsetted;
        for _ in 0..(PIECE_SIZE - first_clear_col) {
            placements.push(tmp_col_offsetted);
            tmp_col_offsetted.roll_right();
        }
        tmp_row_offsetted.roll_down()
    }

    placements
}

/// Return a Vec of this Piece in all its orientations (flips/rotations) and shifts
pub fn get_placements(piece: &Piece) -> Vec<Piece> {
    get_orientations(piece)
        .into_iter()
        .flat_map(|orientation| get_shifts(&orientation))
        .collect()
}
