use crossterm::event::{Event, KeyCode};
use ratatui::layout::{Margin, Rect};
use ratatui::style::{Style, Styled, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Clear, List, ListDirection, Paragraph};
use ratatui::Frame;
use redo::TodoList;

use crate::cursor::{Cursor, CursorMovement};
use crate::event::EventHandler;
use crate::viewport::Viewport;

#[derive(Debug, Default)]
pub struct Editor {
    pub buffer: String,
    pub cursor: Cursor,
    pub viewport: Viewport,
    pub popup_mode: bool,
}

#[derive(Debug, Default)]
pub enum EditorState {
    #[default]
    None,
    Selected,
    Add(String),
    Remove(usize),
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

        if self.popup_mode {
            let popup = Block::bordered().style(Style::default()).green();
            tracing::info!("{:?}", self.buffer);
            let text = Paragraph::new(&*self.buffer);

            let area = frame.area().inner(Margin {
                horizontal: 2,
                vertical: 15,
            });

            frame.render_widget(Clear, area);
            frame.render_widget(popup, area);
            frame.render_widget(
                text,
                area.inner(Margin {
                    horizontal: 1,
                    vertical: 1,
                }),
            );
        }
    }

    pub fn push_char(&mut self, c: char) {
        self.buffer.push(c);
    }
}

impl EventHandler<&mut TodoList, EditorState> for Editor {
    fn handle_event(&mut self, event: &Event, list: &mut TodoList) -> Option<EditorState> {
        if let Event::Key(key) = event {
            if let KeyCode::Char(ch) = key.code {
                self.push_char(ch)
            }
            if let KeyCode::Backspace = key.code {
                self.buffer.pop();
            }
        }

        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => {
                    self.cursor = Cursor::new(self.viewport.x(), 0);
                    return Some(EditorState::Selected);
                }

                KeyCode::Up => self.move_up(1),
                KeyCode::Char('k') => self.move_up(1),

                KeyCode::Down => self.move_down(1, list.len().saturating_sub(1) as u16),
                KeyCode::Char('j') => self.move_up(1),

                KeyCode::Left => self.move_left(1),
                KeyCode::Char('h') => self.move_up(1),

                KeyCode::Right => {
                    let len_line = list.len_line(self.cursor.y as usize).saturating_sub(1);
                    self.move_right(1, len_line as u16);
                    tracing::info!(len_line);
                }
                KeyCode::Char('l') => self.move_up(1),
                KeyCode::Enter => self.popup_mode = !self.popup_mode,

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
