use std::error::Error;

use board_games::connect_four::{game::Game, player::FdPlayer};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdin1 = std::io::stdin().lock();
    let mut stdin2 = std::io::stdin().lock();

    {
        let yellow_player = FdPlayer::new(&mut stdin1, true);
        let red_player = FdPlayer::new(&mut stdin2, true);

        let mut game = Game::new(Box::new(yellow_player), Box::new(red_player));

        game.game_loop()?;
    }

    Ok(())
}
