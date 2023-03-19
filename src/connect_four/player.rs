use std::{
    io::{self, BufRead},
    num::ParseIntError,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

pub trait Player {
    type MoveData;
    type ErrorType;

    fn is_human(&self) -> bool;
    fn get_move(&mut self) -> Result<Self::MoveData, Self::ErrorType>;
}

pub struct FdPlayer<'a> {
    is_human: bool,
    input_stream: &'a mut dyn BufRead,
}

impl<'a> FdPlayer<'a> {
    pub fn new(input_stream: &'a mut dyn BufRead, is_human: bool) -> Self {
        Self {
            input_stream,
            is_human,
        }
    }
}

impl<'a> Player for FdPlayer<'a> {
    type MoveData = usize;
    type ErrorType = Error;

    fn is_human(&self) -> bool {
        self.is_human
    }

    fn get_move(&mut self) -> Result<Self::MoveData, Self::ErrorType> {
        let mut buffer = String::new();

        self.input_stream.read_line(&mut buffer)?;

        let player_move: usize = buffer.trim().parse()?;

        Ok(player_move)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::io::BufReader;

    #[test]
    fn test_new_player() {
        let buffer: Vec<u8> = "0".bytes().collect();
        let mut buffer = BufReader::new(&buffer[..]);

        let mut p1 = FdPlayer::new(&mut buffer, false);

        let player_move = p1.get_move().unwrap();

        assert_eq!(player_move, 0);
    }
}
