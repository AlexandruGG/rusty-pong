mod entity;
mod game;
mod settings;
mod assets;

use crate::game::GameState;
use crate::settings::{WINDOW_HEIGHT, WINDOW_WIDTH};

use tetra::ContextBuilder;

fn main() -> tetra::Result {
    ContextBuilder::new("Rusty Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(GameState::new)
}
