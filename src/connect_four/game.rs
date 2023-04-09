use crate::connect_four::board::GameResult;

use super::{
    board::{self, Board},
    player::Player,
    square::Square,
};

use anyhow::Result;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Board(#[from] board::Error),
}

type ConnectFourPlayer = dyn Player<MoveData = usize>;

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

    pub fn game_loop(&mut self) -> Result<()> {
        println!("Game Start: {:?} to move", self.color_to_be_played);

        loop {
            if self.get_current_player().is_human() {
                println!();
                println!("{}", self.board);

                println!();
                println!("{:?} to move.", self.color_to_be_played);
                println!("{:?}", self.board.list_valid_moves());
                println!("Input the column you wish to play in:");
            }

            loop {
                let player_move = match self.get_player_move() {
                    Ok(m) => m,
                    Err(err) => {
                        println!("{err}");
                        continue;
                    }
                };

                if !self.get_current_player().is_human() {
                    println!();
                    println!("{:?} played {}.", self.color_to_be_played, player_move);
                }

                match self.play_move(player_move) {
                    Ok(_) => {
                        break;
                    }
                    Err(err) => println!("{err}"),
                };
            }

            if let Some(result) = self.board.check_for_win() {
                match result {
                    GameResult::Win(color) => self.print_win(color),
                    GameResult::Draw => self.print_draw(),
                }
                break;
            }
        }

        Ok(())
    }

    // Docstring
    /// Get the player's move retying if the move is invalid.
    fn get_player_move(&mut self) -> Result<usize> {
        // TODO(austin): fix the weird borrow
        let board = &self.board;
        let player = match self.color_to_be_played {
            Square::Yellow => {
                let p = self.yellow_player.as_mut();
                p
            }
            Square::Red => {
                let p = self.red_player.as_mut();
                p
            }
            _ => unreachable!(),
        };

        player.get_move(board)
    }

    fn play_move(&mut self, column: usize) -> Result<()> {
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

    fn print_draw(&self) {
        println!();
        println!();
        println!("{}", self.board);
        println!();
        println!("Draw!");
    }
}
