use std::num::ParseIntError;

use super::{
    board::{self, Board},
    player::{self, Player},
    square::Square,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Board(#[from] board::Error),
    #[error(transparent)]
    Player(#[from] player::Error),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

type ConnectFourPlayer = dyn Player<MoveData = usize, ErrorType = player::Error>;

pub struct Game {
    color_to_be_played: Square,
    board: Board,
    yellow_player: Box<ConnectFourPlayer>,
    red_player: Box<ConnectFourPlayer>,
}

impl Game {
    pub fn new(yellow_player: Box<ConnectFourPlayer>, red_player: Box<ConnectFourPlayer>) -> Self {
        Self {
            color_to_be_played: Square::Yellow,
            board: Board::new(),
            yellow_player,
            red_player,
        }
    }

    pub fn game_loop(&mut self) -> Result<(), Error> {
        println!("Game Start: {:?} to move", self.color_to_be_played);

        loop {
            if self.get_current_player().is_human() {
                println!();
                println!("{}", self.board);

                println!();
                println!("{:?} to move.", self.color_to_be_played);
            }

            loop {
                println!("{:?}", self.board.list_valid_moves());
                println!("Input the column you wish to play in:");

                let player_move = {
                    let player = self.get_current_player_mut();

                    player.get_move()?
                };

                match self.play_move(player_move) {
                    Ok(_) => break,
                    Err(err) => println!("{err}"),
                };
            }

            if let Some(color) = self.board.check_for_win() {
                self.print_win(color);
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

    fn get_current_player(&self) -> &ConnectFourPlayer {
        match self.color_to_be_played {
            Square::Yellow => self.yellow_player.as_ref(),
            Square::Red => self.red_player.as_ref(),
            _ => unreachable!(),
        }
    }

    fn get_current_player_mut(&mut self) -> &mut ConnectFourPlayer {
        match self.color_to_be_played {
            Square::Yellow => self.yellow_player.as_mut(),
            Square::Red => self.red_player.as_mut(),
            _ => unreachable!(),
        }
    }

    fn print_win(&self, color: Square) {
        println!();
        println!();
        println!("{}", self.board);
        println!();
        println!("{color:?} has won!!");
    }
}
