use crossterm::event::{Event, KeyCode};
use ratatui::layout::{Margin, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Clear, List, ListDirection, Paragraph};
use ratatui::Frame;

use crate::cursor::{Cursor, CursorMovement};
use crate::event::EventHandler;
use crate::viewport::Viewport;

#[derive(Debug, Default)]
pub struct SelectionBar {
    pub buffer: String,
    pub viewport: Viewport,

    popup_mode: bool,
    cursor: Cursor,
    scroll: u16,
    names: Vec<String>,
}

#[derive(Debug)]
pub enum SelectionState {
    Selected(usize),
    AddPopup,
    DelPopup,
    Remove(usize),
    AddTodo(String),
    Show(usize),
}

impl EventHandler<(), SelectionState> for SelectionBar {
    fn handle_event(&mut self, event: &Event, _: ()) -> Option<SelectionState> {
        if let Event::Key(key) = event {
            if self.popup_mode {
                match key.code {
                    KeyCode::Char(ch) => {
                        self.buffer.push(ch);
                    }

                    KeyCode::Backspace => {
                        let _ = self.buffer.pop();
                    }

                    KeyCode::Enter => {
                        if self.buffer.is_empty() {
                            return None;
                        }
                        let title = "[".to_string() + &self.buffer + "]";
                        self.popup_mode = false;
                        self.buffer.clear();
                        return Some(SelectionState::AddTodo(title));
                    }

                    KeyCode::Esc => {
                        self.popup_mode = false;
                        return Some(SelectionState::DelPopup);
                    }

                    _ => {}
                }
            }

            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    self.move_up(1);
                    return Some(SelectionState::Show(self.cursor.y as usize));
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    self.move_down(1, self.names.len().saturating_sub(1) as u16);
                    return Some(SelectionState::Show(self.cursor.y as usize));
                }

                KeyCode::Char(' ') => {
                    if !self.names.is_empty() {
                        return Some(SelectionState::Selected(self.cursor.y as usize));
                    }
                }
                KeyCode::Char('x') => {
                    let state = Some(SelectionState::Remove(self.cursor.y as usize));
                    self.remove_name(self.cursor.y as usize);
                    if self.cursor.y as usize > self.names.len().saturating_sub(1) {
                        self.cursor.y = self.cursor.y.saturating_sub(1);
                    }
                    return state;
                }
                KeyCode::Enter => {
                    self.popup_mode = true;
                    return Some(SelectionState::AddPopup);
                }

                KeyCode::Esc => {
                    if self.popup_mode {
                        self.popup_mode = false
                    }
                }

                _ => {}
            }
        }
        None
    }
}

impl SelectionBar {
    pub fn set_names(&mut self, names: Vec<String>) {
        self.names = names
    }

    pub fn draw(&mut self, frame: &mut Frame, selection_area: Rect) {
        let mut names_vec = vec![];
        for item in self
            .names
            .iter()
            .skip(self.scroll as usize)
            .take(self.viewport.y() as usize)
        {
            names_vec.push(item.clone());
        }

        let title = Line::from("Selection").style(Style::default().yellow());
        let list = List::new(names_vec)
            .direction(ListDirection::TopToBottom)
            .block(Block::bordered().red().title_top(title))
            .white();
        frame.render_widget(list, selection_area);
    }

    pub fn draw_popup(&self, frame: &mut Frame) {
        let popup = Block::bordered()
            .style(Style::default())
            .title_top("Adding TodoList")
            .blue();
        let text = Paragraph::new(&*self.buffer).block(popup);

        let area = frame.area().inner(Margin {
            horizontal: 2,
            vertical: 15,
        });

        frame.render_widget(Clear, area);
        frame.render_widget(
            text,
            area.inner(Margin {
                horizontal: 1,
                vertical: 1,
            }),
        );
    }

    pub fn remove_name(&mut self, idx: usize) {
        if self.names.is_empty() || idx > self.names.len() {
            return;
        }
        self.names.remove(idx);
        self.cursor.y = self.cursor.y.saturating_sub(1)
    }

    pub fn cursor_pos(&self) -> (u16, u16) {
        (self.cursor.y, self.cursor.x)
    }
}

impl CursorMovement for SelectionBar {
    fn move_up(&mut self, amount: u16) {
        if self.cursor.y == 0 {
            self.scroll = self.scroll.saturating_sub(1);
        }
        self.cursor.y = self.cursor.y.saturating_sub(amount);
        tracing::debug!("selection_bar: move_up {:?}", self.cursor);
    }

    fn move_down(&mut self, amount: u16, max: u16) {
        self.cursor.y = u16::min(self.cursor.y + amount, max);
        let min = u16::min(max, self.viewport.y());
        if self.cursor.y + amount < min.saturating_sub(1) {
            self.cursor.y += amount;
        }
        if self.cursor.y >= self.viewport.y().saturating_sub(2) && self.cursor.y + self.scroll <= max.saturating_sub(1)
        {
            self.scroll += 1;
        }

        tracing::debug!("selection_bar move_down: {:?}", self.cursor);
    }
}
