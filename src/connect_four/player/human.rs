use crate::connect_four::player::Player;

use std::io::stdin;

use anyhow::Result;

pub struct Human {}

impl Player for Human {
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
