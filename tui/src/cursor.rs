#[derive(Default, Debug)]
pub struct Cursor {
    pub y: u16,
    pub x: u16,
}

pub trait CursorMovement {
    #[allow(unused_variables)]
    fn move_up(&mut self, amount: u16) {}

    #[allow(unused_variables)]
    fn move_down(&mut self, amount: u16, max: u16) {}

    #[allow(unused_variables)]
    fn move_left(&mut self, amount: u16) {}

    #[allow(unused_variables)]
    fn move_right(&mut self, amount: u16, max: u16) {}
}

impl Cursor {
    pub fn new(x: u16, y: u16) -> Self {
        Self { y, x }
    }
}
