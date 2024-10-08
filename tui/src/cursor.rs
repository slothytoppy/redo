#[derive(Default, Debug)]
pub struct Cursor {
    pub y: u16,
    pub x: u16,
}

pub trait CursorMovement {
    fn move_up(&mut self, _amount: u16) {}

    fn move_down(&mut self, _amount: u16, _max: u16) {}

    fn move_left(&mut self, _amount: u16) {}

    fn move_right(&mut self, _amount: u16, _max: u16) {}
}

impl Cursor {
    pub fn new(x: u16, y: u16) -> Self {
        Self { y, x }
    }
}
