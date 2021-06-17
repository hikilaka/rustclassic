use game_loop::game_loop;

mod render;
mod window;

use window::{Window, WindowBuilder};

pub fn main() -> Result<(), String> {
    let window = WindowBuilder::new()
        .title("RustScape by ZP")
        .size(400, 300)
        .build()?;

    game_loop(
        window,
        60,
        50.0,
        |g| {
            g.game.poll_events();

            if g.game.should_close() {
                g.exit();
            }
        },
        |g| {},
    );

    Ok(())
}
