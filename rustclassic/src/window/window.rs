pub trait Window {
    fn poll_events(&mut self);

    fn should_close(&self) -> bool;
}

pub struct WindowBuilder {
    title: Option<String>,
    size: Option<(usize, usize)>,
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self {
            title: None,
            size: None,
        }
    }
    pub fn title(mut self, title: impl ToString) -> Self {
        self.title = Some(title.to_string());
        self
    }
    pub fn size(mut self, width: usize, height: usize) -> Self {
        self.size = Some((width, height));
        self
    }
    pub fn get_title(&self) -> Option<String> {
        self.title.clone()
    }
    pub fn get_size(&self) -> Option<(usize, usize)> {
        self.size
    }
}
