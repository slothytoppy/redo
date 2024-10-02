use std::fmt::Display;
use std::io::{stdout, Stdout, Write};

use crossterm::cursor::{MoveDown, MoveTo, MoveToColumn};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::Clear;

use crate::cursor::Cursor;

#[derive(Debug)]
pub struct Renderer {
    stdout: Stdout,
}

impl Default for Renderer {
    fn default() -> Self {
        Self { stdout: stdout() }
    }
}

impl Renderer {
    /// allows for pipelining multiple commands
    pub fn queue<T: crossterm::Command>(&mut self, command: T) -> &Self {
        let _ = queue!(self.stdout, command);
        self
    }

    pub fn move_to(&mut self, cursor: &Cursor) {
        self.queue(MoveTo(cursor.x(), cursor.y()));
    }

    pub fn draw<T: std::fmt::Display>(&mut self, drawable: &T) {
        self.queue(Print(drawable));
    }

    pub fn draw_vec<T: Display>(&mut self, drawable: &[T]) {
        for val in drawable {
            self.queue(Print(val));
            self.queue(MoveDown(1));
            self.queue(MoveToColumn(0));
        }
    }

    pub fn flush(&mut self) {
        let _ = self.stdout.flush();
    }

    pub fn clear(&mut self, amount: Clear) {
        self.queue(amount);
    }
}
