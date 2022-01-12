use std::collections::{HashMap, HashSet};
use std::fmt;
use std::lazy::SyncLazy;
use std::ops::{Add, Index, IndexMut};

pub static BOARD_LABELS: SyncLazy<HashMap<&str, (usize, usize)>> = SyncLazy::new(|| {
    HashMap::from([
        ("jan", (0, 0)),
        ("feb", (0, 1)),
        ("mar", (0, 2)),
        ("apr", (0, 3)),
        ("may", (0, 4)),
        ("jun", (0, 5)),
        ("jul", (1, 0)),
        ("aug", (1, 1)),
        ("sep", (1, 2)),
        ("oct", (1, 3)),
        ("nov", (1, 4)),
        ("dec", (1, 5)),
        ("1", (2, 0)),
        ("2", (2, 1)),
        ("3", (2, 2)),
        ("4", (2, 3)),
        ("5", (2, 4)),
        ("6", (2, 5)),
        ("7", (2, 6)),
        ("8", (3, 0)),
        ("9", (3, 1)),
        ("10", (3, 2)),
        ("11", (3, 3)),
        ("12", (3, 4)),
        ("13", (3, 5)),
        ("14", (3, 6)),
        ("15", (4, 0)),
        ("16", (4, 1)),
        ("17", (4, 2)),
        ("18", (4, 3)),
        ("19", (4, 4)),
        ("20", (4, 5)),
        ("21", (4, 6)),
        ("22", (5, 0)),
        ("23", (5, 1)),
        ("24", (5, 2)),
        ("25", (5, 3)),
        ("26", (5, 4)),
        ("27", (5, 5)),
        ("28", (5, 6)),
        ("29", (6, 0)),
        ("30", (6, 1)),
        ("31", (6, 2)),
    ])
});

const FILLED_SQUARE_CHAR: &str = "█";
const OPEN_SQUARE_CHAR: &str = "░";
const PIECE_SIZE: usize = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece {
    data: [[u8; PIECE_SIZE]; PIECE_SIZE],
}

impl From<[[u8; PIECE_SIZE]; PIECE_SIZE]> for Piece {
    fn from(data: [[u8; PIECE_SIZE]; PIECE_SIZE]) -> Self {
        Piece { data }
    }
}

impl Index<usize> for Piece {
    type Output = [u8; PIECE_SIZE];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Piece {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Add for Piece {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Piece::from(
            self.data
                .zip(other.data)
                .map(|rows| rows.0.zip(rows.1).map(|cols| cols.0 + cols.1)),
        )
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .into_iter()
                .map(|row| row
                    .into_iter()
                    .map(|col| match col {
                        0 => OPEN_SQUARE_CHAR,
                        _ => FILLED_SQUARE_CHAR,
                    })
                    .collect::<Vec<&str>>()
                    .join(""))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Piece {
    pub fn starting_board() -> Piece {
        Piece::from([
            [0, 0, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1],
        ])
    }

    pub fn playing_pieces() -> Vec<Piece> {
        vec![
            // Rectangle (all these names are my own, btw)
            Piece::from([
                [1, 1, 1, 0, 0, 0, 0],
                [1, 1, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
            ]),
            // Rectangle with a notch
            Piece::from([
                [1, 1, 1, 0, 0, 0, 0],
                [1, 1, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
            ]),
            // Right angle
            Piece::from([
                [1, 1, 1, 0, 0, 0, 0],
                [1, 0, 0, 0, 0, 0, 0],
                [1, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
            ]),
            // S shape
            Piece::from([
                [0, 1, 1, 0, 0, 0, 0],
                [0, 1, 0, 0, 0, 0, 0],
                [1, 1, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
            ]),
            // L shape
            Piece::from([
                [1, 1, 1, 1, 0, 0, 0],
                [1, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
            ]),
            // C shape
            Piece::from([
                [1, 0, 1, 0, 0, 0, 0],
                [1, 1, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
            ]),
            // "Sign Post" shape
            Piece::from([
                [0, 0, 1, 0, 0, 0, 0],
                [1, 1, 1, 1, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
            ]),
            // "Crinkle" shape
            Piece::from([
                [0, 1, 1, 1, 0, 0, 0],
                [1, 1, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
            ]),
        ]
    }

    pub fn coord_for(label: &str) -> Option<&(usize, usize)> {
        BOARD_LABELS.get(label)
    }

    fn col(&self, idx: usize) -> [u8; PIECE_SIZE] {
        self.data.map(|row| row[idx])
    }

    fn transpose(&self) -> Piece {
        Piece::from([
            self.col(0),
            self.col(1),
            self.col(2),
            self.col(3),
            self.col(4),
            self.col(5),
            self.col(6),
        ])
    }

    fn flip_updown(&self) -> Piece {
        Piece::from([
            self[6], self[5], self[4], self[3], self[2], self[1], self[0],
        ])
    }

    fn rotate90(&self) -> Piece {
        self.flip_updown().transpose()
    }

    fn roll_left(&self) -> Piece {
        let t = self.transpose();
        Piece::from([t[1], t[2], t[3], t[4], t[5], t[6], t[0]]).transpose()
    }

    fn roll_right(&self) -> Piece {
        let t = self.transpose();
        Piece::from([t[6], t[0], t[1], t[2], t[3], t[4], t[5]]).transpose()
    }

    fn roll_up(&self) -> Piece {
        Piece::from([
            self[1], self[2], self[3], self[4], self[5], self[6], self[0],
        ])
    }

    fn roll_down(&self) -> Piece {
        Piece::from([
            self[6], self[0], self[1], self[2], self[3], self[4], self[5],
        ])
    }

    pub fn is_flat(&self) -> bool {
        for row in self.data {
            for col in row {
                if col > 1 {
                    return false;
                }
            }
        }
        return true;
    }

    fn shove_left_up(&self) -> Piece {
        let mut tmp = self.clone();

        for _ in 0..PIECE_SIZE {
            if tmp[0].iter().sum::<u8>() == 0 {
                tmp = tmp.roll_up()
            } else {
                break;
            }
        }

        for _ in 0..PIECE_SIZE {
            if tmp.col(0).iter().sum::<u8>() == 0 {
                tmp = tmp.roll_left()
            } else {
                break;
            }
        }

        tmp
    }

    /// Return a Vec of all Pieces that occur if the piece was only rotated and/or flipped.
    /// The piece is shoved up and to the left each time. No duplicates are in the Vec.
    fn get_orientations(&self) -> Vec<Piece> {
        let mut orientations = HashSet::new();
        let mut tmp = self.clone();

        orientations.insert(tmp);

        for _ in 0..3 {
            tmp = tmp.rotate90().shove_left_up();
            orientations.insert(tmp);
        }

        tmp = tmp.flip_updown().shove_left_up();
        orientations.insert(tmp);

        for _ in 0..3 {
            tmp = tmp.rotate90().shove_left_up();
            orientations.insert(tmp);
        }

        orientations.into_iter().collect()
    }

    /// Return a Vec of all Pieces that could occur if the piece was shifted around the board
    /// without rotations or flips
    fn get_shifts(&self) -> Vec<Piece> {
        let mut placements = Vec::new();
        let mut first_clear_col = PIECE_SIZE - 1;
        let mut first_clear_row = PIECE_SIZE - 1;
        let shoved = self.clone().shove_left_up();

        while shoved.col(first_clear_col).iter().sum::<u8>() == 0 {
            first_clear_col -= 1;
        }

        while shoved[first_clear_row].iter().sum::<u8>() == 0 {
            first_clear_row -= 1;
        }

        for row_offset in 0..(PIECE_SIZE - first_clear_row) {
            let mut tmp = shoved.clone();
            for _ in 0..row_offset {
                tmp = tmp.roll_down();
            }
            for _ in 0..(PIECE_SIZE - first_clear_col) {
                placements.push(tmp);
                tmp = tmp.roll_right();
            }
        }

        placements
    }

    /// Return a Vec of this Piece in all its orientations (flips/rotations) and shifts
    pub fn get_placements(&self) -> Vec<Piece> {
        self.get_orientations()
            .into_iter()
            .flat_map(|orientation| orientation.get_shifts())
            .collect()
    }
}
