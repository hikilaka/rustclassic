mod render;
mod window;

use window::{Window, WindowBuilder};

pub fn main() -> Result<(), String> {
    let mut window = WindowBuilder::new()
        .title("RustScape by ZP")
        .size(400, 300)
        .build()?;

    while !window.should_close() {
        window.poll_events();

        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    Ok(())
}
