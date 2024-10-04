use std::io::{stdout, Write};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, List, ListDirection};
use ratatui::{init, restore, DefaultTerminal, Frame};
use redo::todo::TodoListCollection;
use redo::TodoList;

use crate::cursor::{self, Cursor, CursorMovement};
use crate::event::EventHandler;
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

impl SelectionBar {
    pub fn names(&self) -> &Vec<String> {
        &self.names
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let layout = Layout::horizontal([Constraint::Percentage(20)]);
        let [selection_area] = layout.areas(frame.area());
        let list = List::new(self.names.clone())
            .direction(ListDirection::TopToBottom)
            .style(Style::default())
            .red()
            .block(Block::bordered().style(Style::default().red()));
        frame.render_widget(list, selection_area);
    }
}

impl EventHandler for SelectionBar {
    type Event = usize;

    fn handle_event(&mut self, event: &Event) -> Option<Self::Event> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Up => self.move_up(1),
                KeyCode::Down => self.move_down(1),
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

    fn move_down(&mut self, amount: u16) {
        self.cursor.y = u16::min(self.cursor.y + amount, self.names().len().saturating_sub(1) as u16);
        tracing::debug!("selection_bar move_down: {:?}", self.cursor);
    }
}

#[derive(Debug, Default)]
pub struct Editor {
    list: TodoList,
    cursor: Cursor,
    viewport: Viewport,
}

impl Editor {
    pub fn list(&self) -> &TodoList {
        &self.list
    }

    pub fn set_list(&mut self, list: &TodoList) {
        self.list = list.clone()
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let layout = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)]);
        let [_selection_area, editor_area] = layout.areas(frame.area());
        let todos: Vec<String> = self.list.data.iter().map(|todo| todo.to_string()).collect();
        let list = List::new(todos)
            .direction(ListDirection::TopToBottom)
            .style(Style::default())
            .red()
            .block(Block::bordered().style(Style::default().red()));
        frame.render_widget(list, editor_area);
    }
}

impl CursorMovement for Editor {
    fn move_up(&mut self, amount: u16) {
        self.cursor.y = self.cursor.y.saturating_sub(amount);
        tracing::debug!("editor move_up: {:?}", self.cursor);
    }

    fn move_down(&mut self, amount: u16) {
        self.cursor.y = u16::min(self.cursor.y + amount, self.list.data.len().saturating_sub(1) as u16);
        tracing::debug!("editor move_down: {:?}", self.cursor);
    }

    fn move_left(&mut self, amount: u16) {
        self.cursor.x = self.cursor.x.saturating_sub(amount).min(self.viewport.x());
        tracing::debug!("editor move_left: {:?}", self.cursor);
    }

    fn move_right(&mut self, amount: u16) {
        // padding for the todo status + the space at the end
        let padding = 3_u16;
        self.cursor.x = u16::min(
            self.cursor.x + amount,
            self.list.len_line(self.cursor.y as usize) as u16 + padding,
        );
        tracing::debug!("editor move_right: {:?}", self.cursor);
    }
}

impl EventHandler for Editor {
    type Event = bool;

    fn handle_event(&mut self, event: &Event) -> Option<Self::Event> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => {
                    self.cursor = Cursor::new(self.viewport.x(), 0);
                    return Some(true);
                }
                KeyCode::Up => self.move_up(1),
                KeyCode::Down => self.move_down(1),
                KeyCode::Left => self.move_left(1),
                KeyCode::Right => self.move_right(1),
                KeyCode::Char('x') => {
                    if let Some(todo) = self.list.data.get_mut(self.cursor.y as usize) {
                        todo.status.toggle()
                    }
                }

                _ => {}
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct Interface {
    pub collection: TodoListCollection,

    drawable_areas: Vec<Rect>,
    screen_size: Viewport,
    selection_bar: SelectionBar,
    editor: Editor,
    screen_state: ScreenState,
    terminal: DefaultTerminal,
}

impl EventHandler for Interface {
    type Event = bool;

    fn handle_event(&mut self, event: &Event) -> Option<Self::Event> {
        self.handle_resize(event);
        if self.should_quit(event) {
            return Some(true);
        }

        match self.screen_state {
            ScreenState::Selection => {
                if let Some(idx) = self.selection_bar.handle_event(event) {
                    let list = &self.collection.lists[idx];
                    self.editor.set_list(list);
                    self.change_state(ScreenState::Main);
                }
            }

            ScreenState::Main => {
                if self.editor.handle_event(event).unwrap_or(false) {
                    self.change_state(ScreenState::Selection);
                }
            }
        };

        None
    }
}

impl Interface {
    pub fn new(collection: TodoListCollection) -> Self {
        let terminal = init();
        Self {
            drawable_areas: vec![],
            terminal,
            collection,
            selection_bar: SelectionBar::default(),
            editor: Editor::default(),
            screen_size: Viewport::default(),
            screen_state: ScreenState::default(),
        }
    }

    pub fn deinit(&self) {
        restore();
    }

    pub fn set_editor_viewport(&mut self, viewport: Viewport) {
        self.editor.cursor = cursor::Cursor::new(self.editor.viewport.x(), 0);
    }

    pub fn set_editor_list(&mut self, list: TodoList) {
        self.editor.set_list(&list);
    }

    pub fn get_editor_viewport(&self) -> &Viewport {
        &self.editor.viewport
    }

    pub fn set_selection_viewport(&mut self, viewport: Viewport) {
        self.selection_bar.viewport = viewport;
    }

    pub fn get_selection_viewport(&self) -> &Viewport {
        &self.selection_bar.viewport
    }

    pub fn collection_names(&self) -> &Vec<String> {
        self.selection_bar.names()
    }

    pub fn change_state(&mut self, state: ScreenState) {
        self.screen_state = state;
    }

    pub fn update_editor_list(&mut self, list: TodoList) {
        self.editor.list = list;
    }

    pub fn change_collection_names(&mut self, names: Vec<String>) {
        self.selection_bar.names = names;
    }

    pub fn draw(&mut self) {
        _ = self.terminal.draw(|frame| {
            let layout = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)]);
            let [_selection_area, editor_area] = layout.areas(frame.area());

            self.selection_bar.draw(frame);
            self.editor.draw(frame);

            match self.screen_state {
                ScreenState::Selection => {
                    let x = self.selection_bar.cursor.x + 1;
                    let y = self.selection_bar.cursor.y + 1;
                    let position = Position::new(x, y);
                    frame.set_cursor_position(position);
                }

                ScreenState::Main => {
                    let x = self.editor.cursor.x + 1;
                    let y = self.editor.cursor.y + 1;
                    let position = Position::new(x + editor_area.x, y);
                    frame.set_cursor_position(position);
                }
            };
        });
    }

    pub fn flush(&mut self) {
        let _ = stdout().flush();
    }

    pub fn should_quit(&mut self, event: &Event) -> bool {
        let quit_event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL);
        match event {
            Event::Key(key) => *key == quit_event,
            _ => false,
        }
    }

    pub fn handle_resize(&mut self, event: &Event) {
        if let Event::Resize(x, y) = event {
            self.screen_size = Viewport::new(*y, *x);
        }
    }
}
