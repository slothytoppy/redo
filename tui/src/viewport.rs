use std::fmt::Display;

#[derive(Debug, Default)]
pub struct Viewport {
    x: u16,
    y: u16,
}

impl Display for Viewport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "y: {} x: {}", self.y(), self.x())
    }
}

impl Viewport {
    pub fn x(&self) -> u16 {
        self.x
    }

    pub fn y(&self) -> u16 {
        self.y
    }

    pub fn new(y: u16, x: u16) -> Self {
        Self { y, x }
    }
}
