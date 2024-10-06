use crossterm::event::{Event, KeyCode};
use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, List, ListDirection};
use ratatui::Frame;

use crate::cursor::{Cursor, CursorMovement};
use crate::event::EventHandler;

#[derive(Debug, Default)]
pub struct SelectionBar {
    names: Vec<String>,
    pub cursor: Cursor,
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
}

impl EventHandler<&Vec<String>, usize> for SelectionBar {
    fn handle_event(&mut self, event: &Event, _: &Vec<String>) -> Option<usize> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Up => self.move_up(1),
                KeyCode::Down => self.move_down(1, self.names.len().saturating_sub(1) as u16),
                KeyCode::Char(' ') => return Some(self.cursor.y as usize),

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
