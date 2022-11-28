use std::fmt::{Display, Formatter};

// TODO(austin); make this variable
const WIDTH: usize = 7;
const HEIGHT: usize = 6;

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub enum Square {
    Yellow,
    Red,
    #[default]
    Empty,
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let letter_repr = match self {
            Self::Yellow => 'Y',
            Self::Red => 'R',
            Self::Empty => '_',
        };

        write!(f, "{}", letter_repr)
    }
}

pub struct Board {
    board: [[Square; WIDTH]; HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [[Square::Empty; WIDTH]; HEIGHT],
        }
    }
    pub fn play_move(&mut self, column: usize, color: Square) {
        // TODO(austin): clean up with custo error
        let row = (0..HEIGHT)
            .find(|row| self.board[*row][column] == Square::Empty)
            .unwrap();
        self.board[row][column] = color;
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
            write!(f, "{:>2} [", i)?;
            for cell in row {
                write!(f, " {}", cell)?;
            }
            writeln!(f, " ]")?;
        }

        write!(f, " x  ")?;
        for i in 0..WIDTH {
            write!(f, " {}", i)?;
        }
        writeln!(f, "  ")?;

        Ok(())
    }
}
