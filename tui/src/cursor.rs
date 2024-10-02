#[derive(Default, Debug)]
pub struct Cursor {
    y: u16,
    x: u16,
}

#[allow(unused)]
pub enum Direction {
    Up(u16),
    Down(u16),
    Left(u16),
    Right(u16),
}

impl Cursor {
    pub fn new(y: u16, x: u16) -> Self {
        Self { y, x }
    }

    pub fn y(&self) -> u16 {
        self.y
    }

    pub fn x(&self) -> u16 {
        self.x
    }

    pub fn move_dir(&mut self, direction: &Direction) {
        match direction {
            Direction::Up(amount) => self.move_up(*amount),
            Direction::Down(amount) => self.move_down(*amount),
            Direction::Left(amount) => self.move_left(*amount),
            Direction::Right(amount) => self.move_right(*amount),
        }
    }

    pub fn move_up(&mut self, amount: u16) {
        self.y = self.y.saturating_sub(amount);
    }

    pub fn move_down(&mut self, amount: u16) {
        self.y += amount;
    }

    pub fn move_left(&mut self, amount: u16) {
        self.x = self.x.saturating_sub(amount);
    }

    pub fn move_right(&mut self, amount: u16) {
        self.x += amount;
    }
}
