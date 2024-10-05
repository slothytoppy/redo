use std::io::{stdout, Write};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Position};
use ratatui::{init, restore, DefaultTerminal};
use redo::todo::TodoListCollection;

use crate::cursor::{self};
use crate::editor::Editor;
use crate::event::EventHandler;
use crate::selection::SelectionBar;
use crate::viewport::Viewport;

#[derive(Debug, Default)]
pub enum ScreenState {
    #[default]
    Selection,
    Main,
}

#[derive(Debug)]
pub struct Interface {
    pub collection: TodoListCollection,

    selected: usize,
    screen_size: Viewport,
    selection_bar: SelectionBar,
    terminal: DefaultTerminal,

    editor: Editor,
    screen_state: ScreenState,
}

impl Default for Interface {
    fn default() -> Self {
        Self {
            terminal: init(),
            collection: TodoListCollection::default(),
            selected: 0,
            screen_size: Viewport::default(),
            selection_bar: SelectionBar::default(),
            editor: Editor::default(),
            screen_state: ScreenState::default(),
        }
    }
}

impl EventHandler for Interface {
    type Event = bool;
    type Input = ();

    fn handle_event(&mut self, event: &Event, _: &Self::Input) -> Option<Self::Event> {
        self.handle_resize(event);
        if self.should_quit(event) {
            return Some(true);
        }

        match self.screen_state {
            ScreenState::Selection => {
                if let Some(idx) = self.selection_bar.handle_event(event, &()) {
                    self.change_state(ScreenState::Main);
                    self.selected = idx;
                }
            }

            ScreenState::Main => {
                if self
                    .editor
                    .handle_event(event, &self.collection.lists[self.selected])
                    .unwrap_or(false)
                {
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
            terminal,
            collection,

            selected: 0,
            selection_bar: SelectionBar::default(),
            editor: Editor::default(),
            screen_size: Viewport::default(),
            screen_state: ScreenState::default(),
        }
    }

    pub fn draw(&mut self) {
        _ = self.terminal.draw(|frame| {
            let layout = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)]);
            let [selection_area, editor_area] = layout.areas(frame.area());

            self.selection_bar.draw(frame, selection_area);
            self.editor
                .draw(frame, editor_area, &self.collection.lists[self.selected]);

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

    pub fn deinit(&self) {
        restore();
    }

    pub fn set_editor_viewport(&mut self) {
        self.editor.cursor = cursor::Cursor::new(self.editor.viewport.x(), 0);
    }

    pub fn get_editor_viewport(&self) -> &Viewport {
        &self.editor.viewport
    }

    pub fn collection_names(&self) -> &Vec<String> {
        self.selection_bar.names()
    }

    pub fn change_state(&mut self, state: ScreenState) {
        self.screen_state = state;
    }

    pub fn change_collection_names(&mut self, names: Vec<String>) {
        self.selection_bar.set_names(names);
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
