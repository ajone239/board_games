use std::io::stdin;

use anyhow::Result;
use rand::{thread_rng, Rng};

pub trait Player {
    type MoveData;

    fn is_human(&self) -> bool;
    fn get_move(&mut self) -> Result<Self::MoveData>;
}

pub struct HumanPlayer {}

impl Player for HumanPlayer {
    type MoveData = usize;

    fn is_human(&self) -> bool {
        true
    }

    fn get_move(&mut self) -> Result<Self::MoveData> {
        let mut buffer = String::new();

        stdin().read_line(&mut buffer)?;

        let player_move: usize = buffer.trim().parse()?;

        Ok(player_move)
    }
}

pub struct RandomPlayer {}

impl Player for RandomPlayer {
    type MoveData = usize;

    fn is_human(&self) -> bool {
        false
    }

    fn get_move(&mut self) -> Result<Self::MoveData> {
        let mut rng = thread_rng();

        let player_move: usize = rng.gen_range(0..=6);

        Ok(player_move)
    }
}

#[cfg(test)]
mod test {}
