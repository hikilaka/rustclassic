use super::{Window, WindowBuilder};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window as SDLWindow;
use sdl2::{EventPump, Sdl};

pub struct SdlWindow {
    sdl: Sdl,
    window: SDLWindow,
    event_pump: EventPump,
    close_window: bool,
}

impl SdlWindow {
    fn new(sdl: Sdl, window: SDLWindow) -> Result<Self, String> {
        let event_pump = sdl.event_pump()?;

        Ok(SdlWindow {
            sdl,
            window,
            event_pump,
            close_window: false,
        })
    }
}

impl Window for SdlWindow {
    fn poll_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.close_window = true,
                _ => {}
            }
        }
    }
    fn should_close(&self) -> bool {
        self.close_window
    }
}

impl WindowBuilder {
    pub fn build(self) -> Result<SdlWindow, String> {
        let sdl = sdl2::init()?;
        let video_subsys = sdl.video()?;

        let title = match self.get_title() {
            Some(title) => title,
            None => "RustScape".into(),
        };
        let (width, height) = match self.get_size() {
            Some(dimensions) => dimensions,
            None => (300, 400),
        };

        let window = video_subsys
            .window(title.as_str(), width as u32, height as u32)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(SdlWindow::new(sdl, window)?)
    }
}
