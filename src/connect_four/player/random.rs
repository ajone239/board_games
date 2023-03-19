use crate::connect_four::player::Player;

use anyhow::Result;
use rand::{thread_rng, Rng};

pub struct Random {}

impl Player for Random {
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
