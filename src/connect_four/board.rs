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

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Board {
    board: [[Square; WIDTH]; HEIGHT],
}

#[derive(Debug, Eq, PartialEq)]
pub enum GameResult {
    Win(Square),
    Draw,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [[Square::Empty; WIDTH]; HEIGHT],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|square| square == &Square::Empty))
    }

    pub fn is_full(&self) -> bool {
        !self
            .board
            .iter()
            .any(|row| row.iter().any(|square| square == &Square::Empty))
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
        if column > WIDTH - 1 {
            return Err(Error::InvalidMove(column));
        }

        let row = (0..HEIGHT).find(|row| self.board[*row][column] == Square::Empty);

        let row = match row {
            Some(row) => row,
            None => return Err(Error::InvalidMove(column)),
        };

        self.board[row][column] = color;

        Ok(())
    }

    pub fn remove_move(&mut self, column: usize, color: Square) -> Result<(), Error> {
        let row = (0..HEIGHT).rev().find(|row| {
            let mut is_row = self.board[*row][column] == color;
            if *row != HEIGHT - 1 {
                is_row &= self.board[*row + 1][column] == Square::Empty;
            }
            is_row
        });

        let row = match row {
            Some(row) => row,
            None => return Err(Error::InvalidMove(column)),
        };

        self.board[row][column] = Square::Empty;

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

    pub fn is_valid_move(&self, column: usize) -> bool {
        self.board[HEIGHT - 1][column] == Square::Empty
    }

    pub fn eval(&self) -> isize {
        let mut eval = 0;
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let color = self.board[i][j];

                match color {
                    Square::Yellow => eval += self.eval_square(i, j, color),
                    Square::Red => eval -= self.eval_square(i, j, color),
                    _ => continue,
                };
            }
        }

        eval
    }

    fn eval_square(&self, i: usize, j: usize, color: Square) -> isize {
        if color == Square::Empty {
            return 0;
        }

        let mut directions = [Some(0); 4];

        for l in 1..4 {
            /*
             * Check the board with the following mask pattern:
             *
             *  #  #  #
             *   # # #
             *    ###
             *     ####
             */

            let dir_coords = [
                (i + l, j),                      // North
                (i, j + l),                      // East
                (i + l, j + l),                  // North East
                (i + l, j.overflowing_sub(l).0), // North West
            ];

            for (dir, (i, j)) in dir_coords.iter().enumerate() {
                if self.check_in_bound_same_color_or_empty(*i, *j, color)
                    && directions[dir].is_some()
                {
                    if self.board[*i as usize][*j as usize] == color {
                        directions[dir] = directions[dir].map(|d| d + 1);
                    }
                } else {
                    directions[dir] = None;
                }
            }
        }

        // TODO(austin): think harder about this
        directions.iter().fold(1, |acc, dir| acc + dir.unwrap_or(0))
    }

    pub fn check_for_win(&self) -> Option<GameResult> {
        // TODO(austin): Optimize this
        // Check for win
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let color = self.board[i][j];

                if color == Square::Empty {
                    continue;
                }

                if self.check_for_square_win(i, j, color) {
                    return Some(GameResult::Win(color));
                }
            }
        }
        // Check for draw
        if self.is_full() {
            return Some(GameResult::Draw);
        }
        None
    }

    fn check_for_square_win(&self, i: usize, j: usize, color: Square) -> bool {
        if color == Square::Empty {
            return false;
        }

        let mut directions = [true; 4];

        for l in 1..4 {
            /*
             * Check the board with the following mask pattern:
             *
             *  #  #  #
             *   # # #
             *    ###
             *     ####
             */
            let dir_coords = [
                (i + l, j),                      // North
                (i, j + l),                      // East
                (i + l, j + l),                  // North East
                (i + l, j.overflowing_sub(l).0), // North West
            ];

            for (dir, (i, j)) in dir_coords.iter().enumerate() {
                directions[dir] &= self.check_in_bound_same_color(*i, *j, color);
            }
        }

        directions.iter().fold(false, |acc, dir| acc | dir)
    }

    fn check_in_bound_same_color(&self, i: usize, j: usize, color: Square) -> bool {
        if i > HEIGHT - 1 || j > WIDTH - 1 {
            return false;
        }

        self.board[i as usize][j as usize] == color
    }

    fn check_in_bound_same_color_or_empty(&self, i: usize, j: usize, color: Square) -> bool {
        if i > HEIGHT - 1 || j > WIDTH - 1 {
            return false;
        }

        let color_to_check = self.board[i as usize][j as usize];

        color_to_check == color || color_to_check == Square::Empty
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
        assert!(board.is_empty());
        assert!(!board.is_full());
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

    #[test]
    fn test_apply_move() {
        let data = &[
            "_______", "_______", "_______", "_______", "_______", "Y______",
        ];

        let mut board = Board::new();

        board.apply_move(0, Square::Yellow).unwrap();

        assert_eq!(board, Board::new_from_str_vec(data));
    }

    #[test]
    #[should_panic]
    fn test_apply_move_fail() {
        let data = &[
            "Y______", "Y______", "Y______", "Y______", "Y______", "Y______",
        ];

        let mut board = Board::new_from_str_vec(data);

        board.apply_move(0, Square::Yellow).unwrap();
    }

    #[test]
    fn test_remove_move() {
        let player_move = (0, Square::Yellow);
        let mut board = Board::new();

        board.apply_move(player_move.0, player_move.1).unwrap();

        board.remove_move(player_move.0, player_move.1).unwrap();

        assert_eq!(board, Board::new());
    }
    #[test]
    fn test_remove_move_bound_test() {
        let data = &[
            "_______", "Y______", "Y______", "Y______", "Y______", "Y______",
        ];
        let player_move = (0, Square::Yellow);
        let mut board = Board::new_from_str_vec(data);

        board.apply_move(player_move.0, player_move.1).unwrap();

        board.remove_move(player_move.0, player_move.1).unwrap();

        assert_eq!(board, Board::new_from_str_vec(data));
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
    Some(GameResult::Win(Square::Red)))]
    #[case::vert_bm(&[
        "_______",
        "_______",
        "_R_____",
        "_R_____",
        "_R_____",
        "_R_____",
    ], Some(GameResult::Win(Square::Red)))]
    #[case::vert_br(&[
        "_______",
        "_______",
        "______R",
        "______R",
        "______R",
        "______R",
    ], Some(GameResult::Win(Square::Red)))]
    #[case::vert_ml(&[
        "_______",
        "R______",
        "R______",
        "R______",
        "R______",
        "_______",
    ], Some(GameResult::Win(Square::Red)))]
    #[case::vert_mm(&[
        "_______",
        "_R_____",
        "_R_____",
        "_R_____",
        "_R_____",
        "_______",
    ], Some(GameResult::Win(Square::Red)))]
    #[case::vert_mr(&[
        "_______",
        "______R",
        "______R",
        "______R",
        "______R",
        "_______",
    ], Some(GameResult::Win(Square::Red)))]
    #[case::vert_tl(&[
        "R______",
        "R______",
        "R______",
        "R______",
        "_______",
        "_______",
    ], Some(GameResult::Win(Square::Red)))]
    #[case::vert_tm(&[
        "_R_____",
        "_R_____",
        "_R_____",
        "_R_____",
        "_______",
        "_______",
    ], Some(GameResult::Win(Square::Red)))]
    #[case::vert_tr(&[
        "______R",
        "______R",
        "______R",
        "______R",
        "_______",
        "_______",
    ], Some(GameResult::Win(Square::Red)))]
    #[case::horz_bl(&[
        "_______",
        "_______",
        "_______",
        "_______",
        "_______",
        "YYYY___",
    ], Some(GameResult::Win(Square::Yellow)))]
    #[case::horz_bm(&[
        "_______",
        "_______",
        "_______",
        "_______",
        "_______",
        "_YYYY__",
    ], Some(GameResult::Win(Square::Yellow)))]
    #[case::horz_br(&[
        "_______",
        "_______",
        "_______",
        "_______",
        "_______",
        "___YYYY",
    ], Some(GameResult::Win(Square::Yellow)))]
    #[case::horz_ml(&[
        "_______",
        "_______",
        "_______",
        "YYYY___",
        "_______",
        "_______",
    ], Some(GameResult::Win(Square::Yellow)))]
    #[case::horz_mm(&[
        "_______",
        "_______",
        "_______",
        "_YYYY__",
        "_______",
        "_______",
    ], Some(GameResult::Win(Square::Yellow)))]
    #[case::horz_mr(&[
        "_______",
        "_______",
        "_______",
        "___YYYY",
        "_______",
        "_______",
    ], Some(GameResult::Win(Square::Yellow)))]
    #[case::horz_tl(&[
        "YYYY___",
        "_______",
        "_______",
        "_______",
        "_______",
        "_______",
    ], Some(GameResult::Win(Square::Yellow)))]
    #[case::horz_tm(&[
        "_YYYY__",
        "_______",
        "_______",
        "_______",
        "_______",
        "_______",
    ], Some(GameResult::Win(Square::Yellow)))]
    #[case::horz_tr(&[
        "___YYYY",
        "_______",
        "_______",
        "_______",
        "_______",
        "_______",
    ], Some(GameResult::Win(Square::Yellow)))]
    #[case::sw_b(&[
        "_______",
        "_______",
        "___Y___",
        "__Y____",
        "_Y_____",
        "Y______",
    ], Some(GameResult::Win(Square::Yellow)))]
    #[case::sw_t(&[
        "___Y___",
        "__Y____",
        "_Y_____",
        "Y______",
        "_______",
        "_______",
    ], Some(GameResult::Win(Square::Yellow)))]
    #[case::ne_b(&[
        "_______",
        "_______",
        "______Y",
        "_____Y_",
        "____Y__",
        "___Y___",
    ], Some(GameResult::Win(Square::Yellow)))]
    #[case::ne_t(&[
        "______Y",
        "_____Y_",
        "____Y__",
        "___Y___",
        "_______",
        "_______",
    ], Some(GameResult::Win(Square::Yellow)))]
    #[case::se_t(&[
        "Y______",
        "_Y_____",
        "__Y____",
        "___Y___",
        "_______",
        "_______",
    ], Some(GameResult::Win(Square::Yellow)))]
    #[case::se_b(&[
        "_______",
        "_______",
        "Y______",
        "_Y_____",
        "__Y____",
        "___Y___",
    ], Some(GameResult::Win(Square::Yellow)))]
    #[case::nw_b(&[
        "_______",
        "_______",
        "___Y___",
        "____Y__",
        "_____Y_",
        "______Y",
    ], Some(GameResult::Win(Square::Yellow)))]
    #[case::nw_t(&[
        "___Y___",
        "____Y__",
        "_____Y_",
        "______Y",
        "_______",
        "_______",
    ], Some(GameResult::Win(Square::Yellow)))]
    #[case::draw(&[
        "RYRYRYR",
        "RYRYRYR",
        "RYRYRYR",
        "YRYRYRY",
        "YRYRYRY",
        "YRYRYRY",
    ], Some(GameResult::Draw))]
    fn test_check_for_win(#[case] data: &[&str; HEIGHT], #[case] expected: Option<GameResult>) {
        let board = Board::new_from_str_vec(data);

        let test = board.check_for_win();

        assert_eq!(test, expected);
    }

    #[rstest]
    #[case::empty(&["_______", "_______", "_______", "_______", "_______", "_______"], 0)]
    #[case::one_square(&["_______", "_______", "_______", "_______", "_______", "Y______"], 1)]
    #[case::two_row(&["_______", "_______", "_______", "_______", "Y______", "Y______"], 3)]
    #[case::three_row(&["_______", "_______", "_______", "Y______", "Y______", "Y______"], 6)]
    #[case::sw_open(&[
        "_______",
        "_______",
        "___Y___",
        "_______",
        "_Y_____",
        "Y______",
    ], 3 + 2 + 1)]
    #[case::sw_blocked(&[
        "_______",
        "_______",
        "___R___",
        "__Y____",
        "_R_____",
        "R______",
    ], 1 - (1 + 1 + 1))]
    #[case::sw_blocked(&[
        "_______",
        "_______",
        "___Y___",
        "__R____",
        "_Y_____",
        "Y______",
    ], (1 + 1 + 1) - 1)]
    #[case::sw_full(&[
        "_______",
        "_______",
        "___Y___",
        "__RR___",
        "_YRY___",
        "YRRY___",
    ], (1 + 1 + 1 + 1 + 1) - (3 + 3 + 2 + 1 + 3))]
    fn test_eval(#[case] data: &[&str; HEIGHT], #[case] expected: isize) {
        let board = Board::new_from_str_vec(data);

        let test = board.eval();

        assert_eq!(test, expected);
    }

    // Test eval_square
    #[rstest]
    fn test_eval_square() {
        let board = Board::new_from_str_vec(&[
            "_______", "_______", "_______", "_______", "_______", "YY_____",
        ]);

        let eval = board.eval_square(0, 0, Square::Yellow);

        assert_eq!(eval, 2);
    }

    #[test]
    fn test_is_full() {
        let board = Board::new_from_str_vec(&[
            "_______", "_______", "_______", "_______", "_______", "YY_____",
        ]);

        assert_eq!(board.is_full(), false);

        let board = Board::new_from_str_vec(&[
            "YYYYYYY", "YYYYYYY", "YYYYYYY", "YYYYYYY", "YYYYYYY", "YYYYYYY",
        ]);

        assert_eq!(board.is_full(), true);
    }
}
