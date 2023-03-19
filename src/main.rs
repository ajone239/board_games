use board_games::connect_four::{game::Game, player};

use anyhow::Result;

fn main() -> Result<()> {
    let yellow_player = player::HumanPlayer {};
    let red_player = player::RandomPlayer {};

    let mut game = Game::new(Box::new(yellow_player), Box::new(red_player));

    game.game_loop()?;

    Ok(())
}
