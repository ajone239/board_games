use std::error::Error;

use board_games::connect_four::game::Game;

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new();

    game.game_loop()?;

    Ok(())
}
