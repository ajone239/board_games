use anyhow::Result;

mod bot;
mod human;
mod random;

pub use bot::Bot;
pub use human::Human;
pub use random::Random;

pub trait Player {
    type MoveData;

    fn is_human(&self) -> bool;
    fn get_move(&mut self) -> Result<Self::MoveData>;
}
