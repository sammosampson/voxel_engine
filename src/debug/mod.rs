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
pub struct EditorState {
    pub windows_visible: HashMap<String, bool>
}

#[derive(Default)]
pub struct EditorVisibility {
}

#[derive(Default)]
pub struct EditorVisible {
}

impl EditorState {
    pub fn set_window_visibility(&mut self, visible: bool, window_name: String) {
        self.windows_visible.insert(window_name, visible);

    }

    pub fn is_window_visible(&self, window_name: &str) -> bool {
        self.windows_visible.contains_key(window_name) 
            && *self.windows_visible.get(window_name).unwrap()
    }
}
