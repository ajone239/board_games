use std::fmt::{Display, Formatter};

use crate::connect_four::square::Square;

// TODO(austin); make this variable
const WIDTH: usize = 7;
const HEIGHT: usize = 6;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Column {0} is an invalid move.")]
    InvalidMove(usize),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Board {
    board: [[Square; WIDTH]; HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [[Square::Empty; WIDTH]; HEIGHT],
        }
    }

    #[cfg(test)]
    pub fn new_from_str_vec(rows: &[&str; HEIGHT]) -> Self {
        let mut board = [[Square::Empty; WIDTH]; HEIGHT];

        for (i, row) in rows.iter().rev().enumerate() {
            for (j, square) in row.chars().enumerate() {
                let square = match square {
                    'R' => Square::Red,
                    'Y' => Square::Yellow,
                    _ => Square::Empty,
                };
                board[i][j] = square;
            }
        }

        Self { board }
    }

    pub fn apply_move(&mut self, column: usize, color: Square) -> Result<(), Error> {
        let row = (0..HEIGHT).find(|row| self.board[*row][column] == Square::Empty);

        let row = match row {
            Some(row) => row,
            None => return Err(Error::InvalidMove(column)),
        };

        self.board[row][column] = color;

        Ok(())
    }

    pub fn list_valid_moves(&self) -> Vec<usize> {
        let mut open_columns = vec![];

        for (i, square) in self.board[HEIGHT - 1][..].iter().enumerate() {
            if square == &Square::Empty {
                open_columns.push(i);
            }
        }

        open_columns
    }

    pub fn check_for_win(&self) -> Option<Square> {
        // TODO(austin): Optimize this
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let color = self.board[i][j];

                if color == Square::Empty {
                    continue;
                }

                if self.check_for_color_win(i as isize, j as isize, color) {
                    return Some(color);
                }
            }
        }
        None
    }

    fn check_for_color_win(&self, i: isize, j: isize, color: Square) -> bool {
        if color == Square::Empty {
            return false;
        }

        // IDEA(austin): if this returned some sort of eval mapping coordinates to numbers of rows
        // this could be used for the bot
        let mut directions = [true; 8];

        // TODO(austin): once testing is full play with what is needed
        for l in 1..4 {
            // North
            directions[0] &= self.check_in_bound_same_color(i + l, j, color);

            // South
            directions[1] &= self.check_in_bound_same_color(i - l, j, color);

            // East
            directions[2] &= self.check_in_bound_same_color(i, j + l, color);

            // West
            directions[3] &= self.check_in_bound_same_color(i, j - l, color);

            // North East
            directions[4] &= self.check_in_bound_same_color(i + l, j + l, color);

            // North West
            directions[5] &= self.check_in_bound_same_color(i + l, j - l, color);

            // South East
            directions[6] &= self.check_in_bound_same_color(i - l, j + l, color);

            // South West
            directions[7] &= self.check_in_bound_same_color(i - l, j - l, color);
        }

        directions.iter().fold(false, |acc, dir| acc | dir)
    }

    fn check_in_bound_same_color(&self, i: isize, j: isize, color: Square) -> bool {
        if i > HEIGHT as isize - 1 || j > WIDTH as isize - 1 || i < 0 || j < 0 {
            return false;
        }

        self.board[i as usize][j as usize] == color
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: [[Square::default(); WIDTH]; HEIGHT],
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for (i, row) in self.board.iter().enumerate().rev() {
            write!(f, "{i:>2} [")?;
            for cell in row {
                write!(f, " {cell}")?;
            }
            writeln!(f, " ]")?;
        }

        write!(f, " x  ")?;
        for i in 0..WIDTH {
            write!(f, " {i}")?;
        }
        writeln!(f, "  ")?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;

    #[test]
    fn test_new_from_str_vec() {
        let data = &[
            "_______", "_______", "_______", "_______", "_______", "_______",
        ];
        let board = Board::new_from_str_vec(data);

        assert_eq!(board, Board::new());
    }
    #[test]
    fn test_new_from_str_vec_with_moves() {
        let data = &[
            "_______", "_______", "R______", "R______", "R______", "R_YYYY_",
        ];
        let board = Board::new_from_str_vec(data);

        let mut expected_board = Board::new();

        expected_board.apply_move(0, Square::Red).unwrap();
        expected_board.apply_move(0, Square::Red).unwrap();
        expected_board.apply_move(0, Square::Red).unwrap();
        expected_board.apply_move(0, Square::Red).unwrap();

        expected_board.apply_move(2, Square::Yellow).unwrap();
        expected_board.apply_move(3, Square::Yellow).unwrap();
        expected_board.apply_move(4, Square::Yellow).unwrap();
        expected_board.apply_move(5, Square::Yellow).unwrap();

        assert_eq!(board, expected_board);
    }

    #[rstest]
    #[case(&["_______", "_______", "_______", "_______", "_______", "_______"], None)]
    #[case(&["_______", "_______", "_______", "_______", "_______", "R______"], None)]
    #[case(&["_______", "_______", "_______", "_______", "R______", "R______"], None)]
    #[case(&["_______", "_______", "_______", "R______", "R______", "R______"], None)]
    #[case::vert_bl(&[
        "_______",
        "_______",
        "R______",
        "R______",
        "R______",
        "R______"
    ],
    Some(Square::Red))]
    #[case::vert_bm(&[
        "_______",
        "_______",
        "_R_____",
        "_R_____",
        "_R_____",
        "_R_____",
    ], Some(Square::Red))]
    #[case::vert_br(&[
        "_______",
        "_______",
        "______R",
        "______R",
        "______R",
        "______R",
    ], Some(Square::Red))]
    #[case::vert_ml(&[
        "_______",
        "R______",
        "R______",
        "R______",
        "R______",
        "_______",
    ], Some(Square::Red))]
    #[case::vert_mm(&[
        "_______",
        "_R_____",
        "_R_____",
        "_R_____",
        "_R_____",
        "_______",
    ], Some(Square::Red))]
    #[case::vert_mr(&[
        "_______",
        "______R",
        "______R",
        "______R",
        "______R",
        "_______",
    ], Some(Square::Red))]
    #[case::vert_tl(&[
        "R______",
        "R______",
        "R______",
        "R______",
        "_______",
        "_______",
    ], Some(Square::Red))]
    #[case::vert_tm(&[
        "_R_____",
        "_R_____",
        "_R_____",
        "_R_____",
        "_______",
        "_______",
    ], Some(Square::Red))]
    #[case::vert_tr(&[
        "______R",
        "______R",
        "______R",
        "______R",
        "_______",
        "_______",
    ], Some(Square::Red))]
    fn test_check_for_win(#[case] data: &[&str; HEIGHT], #[case] expected: Option<Square>) {
        let board = Board::new_from_str_vec(data);

        let test = board.check_for_win();

        assert_eq!(test, expected);
    }
}
