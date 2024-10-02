use std::fmt::Display;
use std::io::{self, Write};
use std::ops::{Index, IndexMut};

use crossterm::cursor::{MoveDown, MoveTo, MoveToRow};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::Clear;
use redo::todo::{Todo, TodoStatus};
use redo::TodoList;

use crate::cursor::{Cursor, Direction};
use crate::renderer::Renderer;
use crate::viewport::Viewport;

#[derive(Debug, Default)]
pub enum ScreenState {
    #[default]
    Selection,
    Main,
}

#[derive(Debug, Default)]
pub struct SelectionBar {
    names: Vec<String>,
    cursor: Cursor,
    viewport: Viewport,
}

impl SelectionBar {}

#[derive(Debug, Default)]
pub struct Editor {
    list: TodoList,
    cursor: Cursor,
    viewport: Viewport,
}

impl Editor {}

#[derive(Debug, Default)]
pub struct Interface {
    render: Renderer,
    screen_size: Viewport,
    selection_bar: SelectionBar,
    editor: Editor,
    screen_state: ScreenState,
}

impl Interface {
    pub fn change_state(&mut self, state: ScreenState) {
        self.screen_state = state;
    }

    pub fn update_editor_list(&mut self, list: TodoList) {
        self.editor.list = list;
    }

    pub fn change_collection_names(&mut self, names: Vec<String>) {
        self.selection_bar.names = names;
    }

    pub fn handle_event(&mut self, event: &Event) {
        self.handle_resize(event);
        if self.should_quit(event) {}
        match self.screen_state {
            ScreenState::Main => {}
            ScreenState::Selection => {}
        }
    }

    pub fn draw(&mut self) {
        match self.screen_state {
            ScreenState::Main => {}
            ScreenState::Selection => {}
        }
    }

    pub fn move_to(&mut self) {
        match self.screen_state {
            ScreenState::Main => {
                let cursor = &self.editor.cursor;
                let move_to = MoveTo(cursor.x(), cursor.y());
                self.render.queue(move_to);
            }
            ScreenState::Selection => {
                let cursor = &self.selection_bar.cursor;
                let move_to = MoveTo(cursor.x(), cursor.y());
                self.render.queue(move_to);
            }
        }
    }

    pub fn flush(&mut self) {
        self.render.flush();
    }

    pub fn should_quit(&mut self, event: &Event) -> bool {
        let quit_event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL);
        match event {
            Event::Key(key) => {
                if *key == quit_event {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn handle_resize(&mut self, event: &Event) {
        match event {
            Event::Resize(x, y) => {
                self.screen_size = Viewport::new(*y, *x);
            }
            _ => {}
        }
    }
}
