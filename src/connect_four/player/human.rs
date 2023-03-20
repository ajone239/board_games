use crate::connect_four::{board::Board, player::Player};

use std::io::stdin;

use anyhow::Result;

pub struct Human {}

impl Player for Human {
    type MoveData = usize;

    fn is_human(&self) -> bool {
        true
    }

    fn get_move(&mut self, current_board: &Board) -> Result<Self::MoveData> {
        let mut buffer = String::new();

        // The human will see the board, so it isn't needed.
        // May be used for printing in the future
        let _ = current_board;

        stdin().read_line(&mut buffer)?;

        let player_move: usize = buffer.trim().parse()?;

        Ok(player_move)
    }
}
