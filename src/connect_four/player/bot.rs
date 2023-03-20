use crate::connect_four::{board::Board, player::Player};

use anyhow::Result;
use rand::{seq::SliceRandom, thread_rng};

pub struct Bot {}

impl Player for Bot {
    type MoveData = usize;

    fn is_human(&self) -> bool {
        false
    }

    fn get_move(&mut self, current_board: &Board) -> Result<Self::MoveData> {
        let mut rng = thread_rng();
        let moves = current_board.list_valid_moves();

        // SAFETY: A user won't be asked to pick a move if there is a draw.
        let player_move: usize = *moves.choose(&mut rng).unwrap();

        Ok(player_move)
    }
}
