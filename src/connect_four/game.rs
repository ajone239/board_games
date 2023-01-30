use std::{io, num::ParseIntError};

use super::{
    board::{self, Board},
    square::Square,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Board(#[from] board::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

#[derive(Default)]
pub struct Game {
    color_to_be_played: Square,
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        Self {
            color_to_be_played: Square::Yellow,
            board: Board::new(),
        }
    }

    pub fn game_loop(&mut self) -> Result<(), Error> {
        println!("Game Start: {:?} to move", self.color_to_be_played);

        loop {
            println!();
            println!("{}", self.board);

            println!();
            println!("{:?} to move.", self.color_to_be_played);

            loop {
                println!("{:?}", self.board.list_valid_moves());
                println!("Input the column you wish to play in:");

                let player_move = self.get_move()?;

                match self.play_move(player_move) {
                    Ok(_) => break,
                    Err(err) => println!("{err}"),
                };
            }

            if let Some(color) = self.board.check_for_win() {
                println!();
                println!();
                println!("{}", self.board);
                println!();
                println!("{color:?} has won!!");
                break;
            }
        }

        Ok(())
    }

    fn play_move(&mut self, column: usize) -> Result<(), board::Error> {
        self.board.apply_move(column, self.color_to_be_played)?;

        self.color_to_be_played.flip();

        Ok(())
    }

    fn get_move(&self) -> Result<usize, Error> {
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer)?;

        let player_move: usize = buffer.trim().parse()?;

        Ok(player_move)
    }
}
