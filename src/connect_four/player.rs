use std::io::stdin;

use anyhow::Result;

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

#[cfg(test)]
mod test {}
