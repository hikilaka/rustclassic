mod window;
pub use window::*;

#[cfg(not(target_arch = "wasm32"))]
mod sdl_window;
#[cfg(not(target_arch = "wasm32"))]
pub use sdl_window::*;

#[cfg(target_arch = "wasm32")]
mod web_window;
#[cfg(target_arch = "wasm32")]
pub use web_window::*;
