use crossterm::event::{Event, KeyCode};
use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, List, ListDirection};
use ratatui::Frame;
use redo::TodoList;

use crate::cursor::{Cursor, CursorMovement};
use crate::event::EventHandler;
use crate::viewport::Viewport;

#[derive(Debug, Default)]
pub struct Editor {
    pub cursor: Cursor,
    pub viewport: Viewport,
}

impl Editor {
    pub fn draw(&mut self, frame: &mut Frame, editor_area: Rect, list: &TodoList) {
        let mut todos_vec = vec![];
        list.data.iter().for_each(|todo| {
            todos_vec.push(todo.status.to_string() + &todo.data);
        });

        let todos = List::new(todos_vec)
            .direction(ListDirection::TopToBottom)
            .style(Style::default())
            .blue()
            .block(Block::bordered().style(Style::default().white()));
        frame.render_widget(todos, editor_area);
    }
}

impl EventHandler for Editor {
    type Event = bool;
    type Input = TodoList;

    fn handle_event(&mut self, event: &Event, list: &mut Self::Input) -> Option<Self::Event> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => {
                    self.cursor = Cursor::new(self.viewport.x(), 0);
                    return Some(true);
                }

                KeyCode::Up => self.move_up(1),
                KeyCode::Down => self.move_down(1, list.len().saturating_sub(1) as u16),
                KeyCode::Left => self.move_left(1),
                KeyCode::Right => {
                    let len_line = list.len_line(self.cursor.y as usize).saturating_sub(1);
                    self.move_right(1, len_line as u16);
                    tracing::info!(len_line);
                }
                KeyCode::Char(' ') => {
                    if let Some(todo) = list.data.get_mut(self.cursor.y as usize) {
                        todo.status.toggle()
                    }
                }

                _ => {}
            }
        }
        None
    }
}

impl CursorMovement for Editor {
    fn move_up(&mut self, amount: u16) {
        self.cursor.y = self.cursor.y.saturating_sub(amount);
        tracing::debug!("editor move_up: {:?}", self.cursor);
    }

    fn move_down(&mut self, amount: u16, max: u16) {
        self.cursor.y = u16::min(self.cursor.y + amount, max);

        tracing::debug!("editor move_down: {:?}", self.cursor);
    }

    fn move_left(&mut self, amount: u16) {
        self.cursor.x = self.cursor.x.saturating_sub(amount).min(self.viewport.x());
        tracing::debug!("editor move_left: {:?}", self.cursor);
    }

    fn move_right(&mut self, amount: u16, max: u16) {
        // padding for the todo status + the space at the end
        //let padding = 3_u16;
        self.cursor.x = u16::min(self.cursor.x + amount, max);
        tracing::debug!("editor move_right: {:?}", self.cursor);
    }
}
