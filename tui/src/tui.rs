use std::ops::Index;

use crossterm::cursor::{MoveDown, MoveTo, MoveToNextLine, RestorePosition, SavePosition};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use redo::todo::TodoList;

use crate::app::TodoListCollection;

#[derive(Debug, Default)]
pub enum Screen {
    #[default]
    Selection,
    Main,
}

#[derive(Debug)]
pub struct Renderer {
    output: std::io::Stdout,
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            output: std::io::stdout(),
        }
    }
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            output: std::io::stdout(),
        }
    }

    fn draw_selection(&self, collection: &TodoListCollection) {
        let _ = queue!(&self.output, MoveTo(0, 0));
        for (idx, list) in collection.list.iter().enumerate() {
            for todo in &list.data {
                let _ = queue!(&self.output, Print(&todo.data), MoveTo(0, idx.saturating_add(1) as u16));
            }
        }
    }

    pub fn draw(&self, screen: &Screen, collection: &TodoListCollection) {
        let _ = queue!(&self.output, SavePosition, Clear(ClearType::All));
        match screen {
            Screen::Main => {}
            Screen::Selection => {
                self.draw_selection(collection);
            }
        }
        let _ = queue!(&self.output, RestorePosition);
    }
}
