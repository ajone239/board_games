use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Default, Hash)]
pub enum Square {
    Yellow,
    Red,
    #[default]
    Empty,
}

impl Square {
    pub fn flip(&mut self) {
        *self = match self {
            Square::Yellow => Square::Red,
            Square::Red => Square::Yellow,
            _ => Square::Empty,
        }
    }
    pub fn flip_into(&self) -> Self {
        match self {
            Square::Yellow => Square::Red,
            Square::Red => Square::Yellow,
            _ => Square::Empty,
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let letter_repr = match self {
            Self::Yellow => 'Y',
            Self::Red => 'R',
            Self::Empty => '_',
        };

        write!(f, "{letter_repr}")
    }
}

impl Debug for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let letter_repr = match self {
            Self::Yellow => "Yellow",
            Self::Red => "Red",
            Self::Empty => "Empty",
        };

        write!(f, "{letter_repr}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case::red(Square::Red, Square::Yellow)]
    #[case::yellow(Square::Yellow, Square::Red)]
    #[case::empty(Square::Empty, Square::Empty)]
    fn test_flip(#[case] mut given: Square, #[case] expected: Square) {
        given.flip();
        assert_eq!(given, expected);
    }
}
