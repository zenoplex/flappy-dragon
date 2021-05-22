mod constants;
mod game;
mod obstacle;
mod player;

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::game::State;
use bracket_lib::prelude::{main_loop, BError, BTermBuilder};

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Flappy Dragon")
        .with_font("../resources/images/flappy32.png", 32, 32)
        .with_simple_console(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "../resources/images/flappy32.png",
        )
        .with_fancy_console(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "../resources/images/flappy32.png",
        )
        .with_tile_dimensions(16, 16)
        .build()?;

    main_loop(context, State::new())
}
