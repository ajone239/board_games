use board_games::connect_four::{game::Game, player};

use anyhow::Result;

fn main() -> Result<()> {
    let yellow_player = Box::new(player::Human {});
    let red_player = Box::new(player::Bot {});

    let mut game = Game::new(yellow_player, red_player);

    game.game_loop()?;

    Ok(())
}
