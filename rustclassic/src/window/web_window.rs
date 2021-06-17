use super::{Window, WindowBuilder};

pub struct WebWindow;

impl Window for WebWindow {
    fn poll_events(&mut self) {}
    fn should_close(&self) -> bool {
        true
    }
}

impl WindowBuilder {
    pub fn build(self) -> Result<WebWindow, String> {
        Ok(WebWindow)
    }
}
