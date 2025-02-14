mod game;
mod utils;

use game::app::GameApp;
use game::maps::{get_map, get_objects};

fn main() {
    let mut game = GameApp::new(get_map(), get_objects());
    game.run();
}
