mod clock;
mod cycles;

pub use cycles::*;
use std::collections::HashMap;

pub fn initialise_debugging() {
    cycles::initialise();
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Debug {
}


#[derive(Default, Clone, Debug)]
pub struct Editor {
    pub editor_visible: bool,
    pub windows_visible: HashMap<String, bool>
}

impl Editor {
    pub fn set_windows_visibility(&mut self, visible: bool, window_name: String) {
        self.windows_visible.insert(window_name, visible);

    }

    pub fn is_window_visible(&self, window_name: &str) -> bool {
        self.windows_visible.contains_key(window_name) 
            && *self.windows_visible.get(window_name).unwrap()
    }
}
