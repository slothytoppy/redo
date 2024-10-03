use std::io::stdout;

use crossterm::cursor::{MoveTo, MoveToNextLine};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::Clear;
use redo::todo::TodoListCollection;
use redo::TodoList;

use crate::cursor::{self, Cursor, CursorMovement};
use crate::event::EventHandler;
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

impl SelectionBar {
    pub fn names(&self) -> &Vec<String> {
        &self.names
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
    }

    fn move_down(&mut self, amount: u16) {
        if self.cursor.y < self.names().len() as u16 - 1 {
            self.cursor.y += amount;
        }
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

    pub fn set_list(&mut self, list: TodoList) {
        self.list = list
    }
}

impl CursorMovement for Editor {
    fn move_up(&mut self, amount: u16) {
        self.cursor.y = self.cursor.y.saturating_sub(amount);
    }

    fn move_down(&mut self, amount: u16) {
        self.cursor.y = u16::min(self.cursor.y + amount, self.list.data.len() as u16 - 1);
    }

    fn move_left(&mut self, amount: u16) {
        self.cursor.x = self.cursor.x.saturating_sub(amount).max(self.viewport.x());
    }

    fn move_right(&mut self, amount: u16) {
        self.cursor.x = u16::min(
            self.cursor.x + amount,
            self.list.len_line(self.cursor.y as usize) as u16 - 1,
        );
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
                KeyCode::Char(' ') => {
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

#[derive(Debug, Default)]
pub struct Interface {
    render: Renderer,
    screen_size: Viewport,
    selection_bar: SelectionBar,
    editor: Editor,
    screen_state: ScreenState,
    pub collection: TodoListCollection,
}

impl Interface {
    pub fn new(collection: TodoListCollection) -> Self {
        Self {
            collection,
            ..Default::default()
        }
    }

    pub fn set_editor_viewport(&mut self, viewport: Viewport) {
        self.editor.viewport = viewport;
        self.editor.cursor = cursor::Cursor::new(self.editor.viewport.x(), 0);
    }

    pub fn set_editor_list(&mut self, list: TodoList) {
        self.editor.set_list(list);
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

    pub fn handle_event(&mut self, event: &Event) {
        self.handle_resize(event);
        //if self.should_quit(event) {}
        match self.screen_state {
            ScreenState::Main => {
                if self.editor.handle_event(event).unwrap_or(false) {
                    self.change_state(ScreenState::Selection);
                }
                self.render.move_to(&self.editor.cursor);
            }
            ScreenState::Selection => {
                if let Some(idx) = self.selection_bar.handle_event(event) {
                    let list = &self.collection.lists[idx];
                    self.editor.set_list(list.clone());
                    self.change_state(ScreenState::Main);
                }
                self.render.move_to(&self.selection_bar.cursor);
            }
        }
    }

    fn draw_selection_screen(&self) {
        self.render.move_to(&cursor::Cursor::new(0, 0));
        for (idx, name) in self.selection_bar.names.iter().enumerate() {
            if idx as u16 > self.selection_bar.viewport.y() {
                return;
            }
            self.render.queue(Print(name)).queue(MoveToNextLine(1));
        }
        self.render.move_to(&self.selection_bar.cursor);
    }

    fn draw_main_screen(&self) {
        tracing::info!("{}", self.editor.list);

        for (idx, name) in self.editor.list().data.iter().enumerate() {
            if idx as u16 > self.editor.viewport.y() {
                return;
            }
            queue!(
                std::io::stdout(),
                MoveTo(self.editor.viewport.x(), idx as u16),
                Print(name)
            )
            .expect("printing to stdout somehow failed????? pepega");
        }
    }

    pub fn draw(&mut self) {
        queue!(stdout(), Clear(crossterm::terminal::ClearType::All)).ok();
        self.draw_selection_screen();
        self.draw_main_screen();
        match self.screen_state {
            ScreenState::Selection => queue!(
                stdout(),
                MoveTo(self.selection_bar.cursor.x, self.selection_bar.cursor.y)
            )
            .ok(),
            ScreenState::Main => queue!(stdout(), MoveTo(self.editor.cursor.x, self.editor.cursor.y)).ok(),
        };
    }

    pub fn flush(&mut self) {
        self.render.flush();
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
