#[derive(Debug, Clone, Copy)]
pub struct Colour {
    r: u8,
    g: u8,
    b: u8
}

impl Colour {
    pub fn green() -> Self {
        Self {
            r: 0,
            g: 255,
            b: 0
        }
    }

    pub fn grey() -> Self {
        Self {
            r: 150,
            g: 150,
            b: 150
        }
    }
}

impl Into<[f32; 3]> for Colour {
    fn into(self) -> [f32; 3] {
        [   
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0
        ]
    }
}