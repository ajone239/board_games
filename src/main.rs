use board_games::connect_four::{game::Game, player::HumanPlayer};

use anyhow::Result;

fn main() -> Result<()> {
    let yellow_player = HumanPlayer {};
    let red_player = HumanPlayer {};

    let mut game = Game::new(Box::new(yellow_player), Box::new(red_player));

    game.game_loop()?;

    Ok(())
}
