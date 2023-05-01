pub mod hexgame;
use hexgame::game::Game;

fn main() {
    let mut game = Game::new(3);
    game.play();
}
