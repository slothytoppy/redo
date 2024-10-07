use crossterm::event::{Event, KeyCode};
use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, List, ListDirection};
use ratatui::Frame;

use crate::cursor::{Cursor, CursorMovement};
use crate::event::EventHandler;

#[derive(Debug, Default)]
pub struct SelectionBar {
    pub cursor: Cursor,
    pub buffer: String,
    pub adding_mode: bool,

    names: Vec<String>,
}

impl SelectionBar {
    pub fn names(&self) -> &Vec<String> {
        &self.names
    }

    pub fn set_names(&mut self, names: Vec<String>) {
        self.names = names
    }

    pub fn draw(&mut self, frame: &mut Frame, selection_area: Rect) {
        let list = List::new(self.names.clone())
            .direction(ListDirection::TopToBottom)
            .style(Style::default())
            .red()
            .block(Block::bordered().style(Style::default().red()));
        frame.render_widget(list, selection_area);
    }

    pub fn remove_name(&mut self, idx: usize) {
        if self.names.is_empty() {
            return;
        }
        self.names.remove(idx);
        self.cursor.y = self.cursor.y.saturating_sub(1)
    }
}

impl EventHandler<(), usize> for SelectionBar {
    fn handle_event(&mut self, event: &Event, _: ()) -> Option<usize> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Up => self.move_up(1),
                KeyCode::Down => self.move_down(1, self.names.len().saturating_sub(1) as u16),
                KeyCode::Char(' ') => return Some(self.cursor.y as usize),
                KeyCode::Char('x') => self.remove_name(self.cursor.y as usize),
                KeyCode::Char(ch) => {
                    if self.adding_mode {
                        self.buffer.push(ch);
                    }
                }

                KeyCode::Enter => {
                    self.adding_mode = !self.adding_mode;
                }

                _ => {}
            }
        }
        None
    }
}

impl CursorMovement for SelectionBar {
    fn move_up(&mut self, amount: u16) {
        self.cursor.y = self.cursor.y.saturating_sub(amount);
        tracing::debug!("selection_bar: move_up {:?}", self.cursor);
    }

    fn move_down(&mut self, amount: u16, max: u16) {
        self.cursor.y = u16::min(self.cursor.y + amount, max);
        tracing::debug!("selection_bar move_down: {:?}", self.cursor);
    }
}
