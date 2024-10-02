#[derive(Debug, Default)]
pub struct Viewport {
    x: u16,
    y: u16,
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
