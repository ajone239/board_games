use std::collections::HashMap;

use crate::connect_four::{board::Board, player::Player, square::Square};

use anyhow::Result;
use rand::{seq::SliceRandom, thread_rng};

pub struct Bot {}

struct GameNode {
    color: Square
    player_move: usize,
    evaluation: isize,
    children: Vec<GameNode>,
}

struct GameTree {
    tree: HashMap<Board, GameNode>,
}

impl Player for Bot {
    type MoveData = usize;

    fn is_human(&self) -> bool {
        false
    }

    fn get_move(&mut self, current_board: &Board) -> Result<Self::MoveData> {
        if current_board.is_empty() {
            return Ok(self.get_random_move(current_board));
        }

        Ok(0)
    }
}

impl Bot {
    fn get_random_move(&self, current_board: &Board) -> usize {
        let mut rng = thread_rng();
        let moves = current_board.list_valid_moves();

        // SAFETY: A user won't be asked to pick a move if there is a draw.
        *moves.choose(&mut rng).unwrap()
    }
}
