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

    adding_mode: bool,
    names: Vec<String>,
}

#[derive(Debug)]
pub enum SelectionState {
    Selected(usize),
    Adding,
    Remove(usize),
}

impl SelectionBar {
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
        if self.names.is_empty() || idx > self.names.len() {
            return;
        }
        self.names.remove(idx);
        self.cursor.y = self.cursor.y.saturating_sub(1)
    }
}

impl EventHandler<(), SelectionState> for SelectionBar {
    fn handle_event(&mut self, event: &Event, _: ()) -> Option<SelectionState> {
        if let Event::Key(key) = event {
            if self.adding_mode {
                if let KeyCode::Char(ch) = key.code {
                    if self.adding_mode {
                        self.buffer.push(ch);
                    }
                    return None;
                }
            }

            match key.code {
                KeyCode::Up => self.move_up(1),
                KeyCode::Down => self.move_down(1, self.names.len().saturating_sub(1) as u16),
                KeyCode::Char(' ') => return Some(SelectionState::Selected(self.cursor.y as usize)),
                KeyCode::Char('x') => {
                    let state = Some(SelectionState::Remove(self.cursor.y as usize));
                    self.remove_name(self.cursor.y as usize);
                    if self.cursor.y as usize > self.names.len() - 1 {
                        self.cursor.y = self.cursor.y.saturating_sub(1);
                    }
                    return state;
                }
                KeyCode::Enter => {
                    if self.adding_mode {
                        self.adding_mode = false;
                        return Some(SelectionState::Adding);
                    }
                    self.adding_mode = true;
                }

                KeyCode::Esc => {
                    if self.adding_mode {
                        self.adding_mode = false
                    }
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
