use crossterm::event::{Event, KeyCode};
use ratatui::layout::{Margin, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Clear, List, ListDirection, Paragraph};
use ratatui::Frame;
use redo::TodoList;

use crate::cursor::{Cursor, CursorMovement};
use crate::event::EventHandler;
use crate::viewport::Viewport;

#[derive(Debug, Default, Clone)]
pub struct Editor {
    pub buffer: String,
    pub cursor: Cursor,
    pub viewport: Viewport,
    pub popup_mode: bool,

    scroll: u16,
}

// #[allow(dead_code] because parts of the enum are detected as "unused" but theyre used in tui.rs
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum EditorState {
    None,
    AddPopup,
    DelPopup,
    Selected,
    Add(String),
    Remove(usize),
}

impl EventHandler<&mut TodoList, EditorState> for Editor {
    fn handle_event(&mut self, event: &Event, list: &mut TodoList) -> Option<EditorState> {
        if self.popup_mode {
            if let Event::Key(key) = event {
                match key.code {
                    KeyCode::Esc => {
                        self.popup_mode = false;
                        self.buffer.clear();
                        return Some(EditorState::DelPopup);
                    }
                    KeyCode::Char(ch) => self.push_char(ch),
                    KeyCode::Backspace => _ = self.buffer.pop(),
                    KeyCode::Enter => {
                        self.popup_mode = false;
                        let title = self.buffer.clone();
                        self.buffer.clear();
                        return Some(EditorState::Add(title));
                    }

                    _ => {}
                };
            }
        }

        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => {
                    self.cursor = Cursor::new(0, 0);
                    return Some(EditorState::None);
                }

                KeyCode::Up | KeyCode::Char('k') => self.move_up(1),
                KeyCode::Down | KeyCode::Char('j') => {
                    self.move_down(1, list.len() as u16);
                }
                KeyCode::Left | KeyCode::Char('h') => self.move_left(1),
                KeyCode::Right | KeyCode::Char('l') => {
                    let max = list.len_line(self.cursor.y as usize);
                    self.move_right(1, max as u16);
                    tracing::info!(max);
                }

                KeyCode::Enter => {
                    self.popup_mode = true;
                    return Some(EditorState::AddPopup);
                }
                KeyCode::Char('x') => {
                    let state = Some(EditorState::Remove(self.cursor.y as usize));
                    return state;
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

impl Editor {
    pub fn draw(&mut self, frame: &mut Frame, editor_area: Rect, list: Option<&TodoList>) {
        let mut todos_vec = vec![];
        if let Some(list) = list {
            for item in list
                .data
                .iter()
                .skip(self.scroll as usize)
                .take(self.viewport.y() as usize)
            {
                todos_vec.push(item.status.to_string() + " " + &item.data);
            }
        };

        let title = Line::from("Selection").style(Style::default().yellow());
        let todos = List::new(todos_vec)
            .direction(ListDirection::TopToBottom)
            .blue()
            .block(Block::bordered().white().title_top(title));
        frame.render_widget(todos, editor_area);
    }

    pub fn draw_popup(&self, frame: &mut Frame) {
        if self.popup_mode {
            let popup = Block::bordered()
                .style(Style::default())
                .green()
                .title_top("Adding Todo");
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
    }

    pub fn push_char(&mut self, c: char) {
        self.buffer.push(c);
    }
}

impl CursorMovement for Editor {
    fn move_up(&mut self, amount: u16) {
        if self.cursor.y == 0 {
            self.scroll = self.scroll.saturating_sub(1);
        }
        self.cursor.y = self.cursor.y.saturating_sub(amount);
        tracing::debug!("editor move_up: {:?}", self.cursor);
    }

    fn move_down(&mut self, amount: u16, max: u16) {
        let min = u16::min(max, self.viewport.y());
        if self.cursor.y + amount < min {
            self.cursor.y += amount;
        }
        if self.cursor.y >= self.viewport.y().saturating_sub(2) && self.cursor.y + self.scroll <= max.saturating_sub(1)
        {
            self.scroll += 1;
        }
        tracing::info!("cursor {:?} scroll: {:?}", self.cursor, self.scroll);
    }

    fn move_left(&mut self, amount: u16) {
        self.cursor.x = self.cursor.x.saturating_sub(amount);
        tracing::debug!("editor move_left: {:?}", self.cursor);
    }

    fn move_right(&mut self, amount: u16, max: u16) {
        // padding for the todo status + the space at the end
        //let padding = 3_u16;
        if self.cursor.x + amount < max {
            self.cursor.x += amount;
        }
        tracing::debug!("editor move_right: {:?}", self.cursor);
    }
}
