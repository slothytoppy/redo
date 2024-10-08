use std::io::{stdout, Write};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Position};
use ratatui::{init, restore, DefaultTerminal};
use redo::todo::TodoListCollection;
use redo::TodoList;

use crate::cursor::{self};
use crate::editor::{Editor, EditorState};
use crate::event::EventHandler;
use crate::selection::{SelectionBar, SelectionState};
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
        let terminal = init();
        let screen_size = ratatui::Terminal::size(&terminal).unwrap_or_default();
        let viewport = Viewport::new(screen_size.height, screen_size.width);

        Self {
            terminal,
            collection: TodoListCollection::default(),
            selected: 0,
            screen_size: viewport,
            selection_bar: SelectionBar::default(),
            editor: Editor::default(),
            screen_state: ScreenState::default(),
        }
    }
}

pub enum InterfaceState {
    Quit(Option<String>),
}

impl EventHandler<(), InterfaceState> for Interface {
    fn handle_event(&mut self, event: &Event, _: ()) -> Option<InterfaceState> {
        self.handle_resize(event);
        if self.should_quit(event) {
            return Some(InterfaceState::Quit(None));
        }

        match self.screen_state {
            ScreenState::Selection => {
                if let Some(state) = self.selection_bar.handle_event(event, ()) {
                    match state {
                        SelectionState::Show(idx) => {
                            self.selected = idx;
                        }
                        SelectionState::Adding => {
                            if let Event::Key(key) = event {
                                if key.code == KeyCode::Enter {
                                    // asserts that if you've pressed enter then enter again that
                                    // you've filled the buffer with at least a char
                                    // otherwise it trips the assert
                                    if self.selection_bar.buffer.is_empty() {
                                        return Some(InterfaceState::Quit(Some("Error: Enter was pressed twice, the app assumes that you've pressed at least one key in order to fill up the buffer to make a todo list".to_string())));
                                    }
                                    //assert!(!self.selection_bar.buffer.is_empty());

                                    let title = "[".to_string() + &self.selection_bar.buffer + "]";
                                    self.collection.push(TodoList::new(title, ""));
                                    self.selection_bar.set_names(self.collection_names());
                                    self.selection_bar.buffer.clear();
                                }
                            }
                        }
                        SelectionState::Selected(idx) => {
                            self.change_state(ScreenState::Main);
                            self.selected = idx;
                        }
                        SelectionState::Remove(idx) => {
                            if idx == 0 && self.collection.lists.is_empty() {
                                return None;
                            }
                            self.collection.lists.remove(idx);
                        }
                    }
                }
            }

            ScreenState::Main => {
                let result = self
                    .editor
                    .handle_event(event, &mut self.collection.lists[self.selected])
                    .unwrap_or_default();
                match result {
                    EditorState::Selected => self.change_state(ScreenState::Selection),
                    EditorState::Add(data) => self.collection.lists[self.selected].push_str(&data),
                    EditorState::Remove(idx) => {
                        let list = &mut self.collection.lists[self.selected];
                        if idx == 0 && list.is_empty() {
                            self.change_state(ScreenState::Selection);
                            return None;
                        }
                        list.data.remove(idx);
                    }

                    _ => {}
                };
            }
        };

        None
    }
}

impl Interface {
    pub fn new(collection: TodoListCollection) -> Self {
        let terminal = init();
        let screen_size = ratatui::Terminal::size(&terminal).unwrap_or_default();
        let viewport = Viewport::new(screen_size.height, screen_size.width);

        Self {
            terminal,
            collection,

            selected: 0,
            selection_bar: SelectionBar::default(),
            editor: Editor::default(),
            screen_size: viewport,
            screen_state: ScreenState::default(),
        }
    }

    pub fn draw(&mut self) {
        _ = self.terminal.draw(|frame| {
            let layout = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)]);
            let [selection_area, editor_area] = layout.areas(frame.area());

            let list = self.collection.lists.get(self.selected);
            self.editor.draw(frame, editor_area, list);
            self.selection_bar.draw(frame, selection_area);

            match self.screen_state {
                ScreenState::Selection => {
                    // have to do x+1, else it puts the cursor at | instead of [
                    let (y, x) = self.selection_bar.cursor_pos();
                    let position = Position::new(x + 1, y + 1);
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

    pub fn add_todo(&mut self, content: &str) {
        let list = self.collection.get_mut_todo_list(self.selected);
        if let Some(list) = list {
            list.push_str(content)
        }
    }

    pub fn remove_todo(&mut self) {
        let list = self.collection.get_mut_todo_list(self.selected);
        if let Some(list) = list {
            list.data.remove(self.editor.cursor.x as usize);
        }
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

    pub fn collection_names(&self) -> Vec<String> {
        let mut tmp = vec![];
        self.collection
            .lists
            .iter()
            .for_each(|list| tmp.push(list.title.clone()));
        tmp
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
