use colored::{ColoredString, Colorize};
use std::fmt;
use std::ops::{Add, Index, IndexMut};

const FILLED_SQUARE_CHAR: &str = "█";
const OPEN_SQUARE_CHAR: &str = "░";
const INVALID_BOARD_LABEL: &str = "?";
pub const PIECE_SIZE: usize = 7;

#[rustfmt::skip]
const BOARD_LABELS: [[&str; PIECE_SIZE]; PIECE_SIZE] = [
    ["jan", "feb", "mar", "apr", "may", "jun", INVALID_BOARD_LABEL,],
    ["jul", "aug", "sep", "oct", "nov", "dec", INVALID_BOARD_LABEL,],
    ["1", "2", "3", "4", "5", "6", "7",],
    ["8", "9", "10", "11", "12", "13", "14",],
    ["15", "16", "17", "18", "19", "20", "21",],
    ["22", "23", "24", "25", "26", "27", "28",],
    ["29", "30", "31", INVALID_BOARD_LABEL, INVALID_BOARD_LABEL, INVALID_BOARD_LABEL, INVALID_BOARD_LABEL,],
];

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
            "{}\n",
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

    /// Returns an Iterator of items like `((row_idx, col_idx), label)`
    ///
    /// For example, `((0, 3), "apr")`
    fn get_label_coords<'a>() -> impl Iterator<Item = ((usize, usize), &'a str)> {
        BOARD_LABELS
            .into_iter()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.into_iter()
                    .enumerate()
                    .map(|(col_idx, label)| ((row_idx, col_idx), label))
                    .collect::<Vec<_>>()
            })
            .filter(|(_, label)| !INVALID_BOARD_LABEL.eq(*label))
    }

    pub fn get_labels<'a>() -> impl Iterator<Item = &'a str> {
        Piece::get_label_coords().map(|(_, label)| label)
    }

    fn coord_for(label: &str) -> Option<(usize, usize)> {
        Piece::get_label_coords()
            .find(|(_, item_label)| item_label.eq(&label))
            .map(|(coord, _)| coord)
    }

    pub fn mark_coord_for(&mut self, label: &str) {
        let coord = Piece::coord_for(label);
        if let Some((row_idx, col_idx)) = coord {
            self[row_idx][col_idx] = 1;
        }
    }

    pub fn col(&self, idx: usize) -> [u8; PIECE_SIZE] {
        self.data.map(|row| row[idx])
    }

    fn transpose(&mut self) {
        self.data = [
            self.col(0),
            self.col(1),
            self.col(2),
            self.col(3),
            self.col(4),
            self.col(5),
            self.col(6),
        ];
    }

    pub fn flip_updown(&mut self) {
        self.data = [
            self[6], self[5], self[4], self[3], self[2], self[1], self[0],
        ];
    }

    pub fn rotate90(&mut self) {
        self.flip_updown();
        self.transpose();
    }

    fn roll_left(&mut self) {
        self.transpose();
        self.roll_up();
        self.transpose();
    }

    pub fn roll_right(&mut self) {
        self.transpose();
        self.roll_down();
        self.transpose();
    }

    fn roll_up(&mut self) {
        self.data = [
            self[1], self[2], self[3], self[4], self[5], self[6], self[0],
        ];
    }

    pub fn roll_down(&mut self) {
        self.data = [
            self[6], self[0], self[1], self[2], self[3], self[4], self[5],
        ];
    }

    pub fn shove_left_up(&mut self) {
        for _ in 0..PIECE_SIZE {
            if self[0].iter().sum::<u8>() == 0 {
                self.roll_up()
            } else {
                break;
            }
        }

        for _ in 0..PIECE_SIZE {
            if self.col(0).iter().sum::<u8>() == 0 {
                self.roll_left()
            } else {
                break;
            }
        }
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
}

pub fn print_solution(pieces: Vec<&Piece>) {
    let colors = [
        (31, 119, 180),  // blue
        (255, 127, 14),  // orange
        (44, 160, 44),   // green
        (214, 39, 40),   // red
        (148, 103, 189), // purple
        (227, 119, 194), // pink
        (188, 189, 34),  // yellow
        (23, 190, 207),  // cyan
    ];

    let mut board = Piece::starting_board().data.map(|row| {
        row.map(|col| match col {
            0 => OPEN_SQUARE_CHAR.white(),
            _ => "".normal(),
        })
    });

    for (piece, (r, g, b)) in pieces.into_iter().zip(colors) {
        let ons = piece
            .data
            .into_iter()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.into_iter()
                    .enumerate()
                    .map(|(col_idx, item)| ((row_idx, col_idx), item))
                    .collect::<Vec<_>>()
            })
            .filter(|(_, item)| item.eq(&1))
            .map(|(coords, _)| coords)
            .collect::<Vec<_>>();
        for (row_idx, col_idx) in ons {
            board[row_idx][col_idx] = FILLED_SQUARE_CHAR.truecolor(r, g, b);
        }
    }

    let s = board
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|cs| cs.to_string())
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}\n", s);
}
