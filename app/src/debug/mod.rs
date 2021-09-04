mod clock;
mod cycles;

pub use cycles::*;

pub fn initialise_debugging() {
    cycles::initialise();
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Debug {
}


#[derive(Default, Clone, Copy, Debug)]
pub struct Editor {
    pub editor_visible: bool
}
