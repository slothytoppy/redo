use std::io::{stdout, Write};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Flex, Layout, Position, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Padding, Paragraph};
use ratatui::{init, restore, DefaultTerminal, Frame};
use redo::todo::TodoListCollection;
use redo::TodoList;

use crate::editor::{Editor, EditorState};
use crate::event::EventHandler;
use crate::selection::{SelectionBar, SelectionState};
use crate::viewport::Viewport;

#[derive(Debug, Default)]
pub enum ScreenState {
    #[default]
    Selection,
    Editor,
    Help,
}

#[derive(Debug, Clone, Copy)]
pub enum PopupState {
    Editor,
    Selection,
}

#[derive(Debug, Default)]
pub struct HelpScreen {
    active: bool,
}

impl HelpScreen {
    pub fn draw(&self, help_area: Rect, frame: &mut Frame) {
        let help_vec: Vec<Line> = vec![
            " Selection Mode "
                .fg(Color::Rgb(255, 255, 255))
                .bg(Color::Rgb(183, 72, 101))
                .bold()
                .into(),
            "".into(),
            "Up/k            Move up   ".into(),
            "Down/j          Move down  ".into(),
            "Right/l         Move right ".into(),
            "Left/h          Move Left ".into(),
            "Space           Select List".into(),
            "Enter           Create List".into(),
            "Esc             Leave Popup".into(),
        ];

        let [layout] = Layout::vertical([Constraint::Length(help_vec.len() as u16 + 6)])
            .flex(Flex::Center)
            .areas(help_area);
        let [layout] = Layout::horizontal([Constraint::Length(50)])
            .flex(Flex::Center)
            .areas(layout);
        let help = Paragraph::new(help_vec)
            .block(Block::default().borders(Borders::ALL).padding(Padding::new(5, 5, 2, 2)))
            .style(Style::default())
            .blue()
            .centered();
        frame.render_widget(help, layout);
    }
}

#[derive(Debug)]
pub struct Interface {
    pub collection: TodoListCollection,

    selected_list: usize,
    screen_size: Viewport,
    selection_bar: SelectionBar,
    terminal: DefaultTerminal,

    editor: Editor,
    screen_state: ScreenState,

    popups: Vec<PopupState>,
    help_screen: HelpScreen,
}

impl Interface {
    pub fn handle_selection_bar(&mut self, event: &Event) {
        if let Some(state) = self.selection_bar.handle_event(event, self.collection_names()) {
            match state {
                SelectionState::DelPopup => _ = self.popups.pop(),
                SelectionState::Show(idx) => {
                    self.selected_list = idx;
                }
                SelectionState::AddPopup => {
                    self.popups.push(PopupState::Selection);
                }
                SelectionState::Selected(idx) => {
                    self.change_state(ScreenState::Editor);
                    self.selected_list = idx;
                }
                SelectionState::AddTodo(title) => {
                    self.collection.push(TodoList::new(title, ""));
                    //self.selection_bar.set_names(self.collection_names());
                    self.popups.pop();
                }
                SelectionState::Remove(idx) => {
                    if idx == 0 && self.collection.lists.is_empty() {
                        return;
                    }
                    self.collection.lists.remove(idx);
                    self.selected_list = self.selected_list.saturating_sub(1);
                }
            };
        };
    }

    pub fn handle_editor(&mut self, event: &Event) {
        let Some(result) = self
            .editor
            .handle_event(event, &mut self.collection.lists[self.selected_list])
        else {
            return;
        };
        match result {
            EditorState::Selected => {
                if !self.collection.lists[self.selected_list].is_empty() {
                    self.change_state(ScreenState::Selection);
                }
                tracing::info!("Selected: {:?}", self.collection.lists[self.selected_list]);
            }
            EditorState::AddPopup => self.popups.push(PopupState::Editor),
            EditorState::Add(data) => {
                if !self.collection.lists[self.selected_list].is_empty() {
                    self.editor.cursor.y += 1;
                }
                self.collection.lists[self.selected_list].push_str(&data);
            }
            EditorState::Remove(idx) => {
                let list = &mut self.collection.lists[self.selected_list];
                assert!(list.data.len() > idx);
                list.data.remove(idx);
                if list.data.len().saturating_sub(1) < self.editor.cursor.y as usize {
                    self.editor.cursor.y = self.editor.cursor.y.saturating_sub(1);
                }
                if list.is_empty() {
                    self.change_state(ScreenState::Selection);
                }
            }
            EditorState::None => self.change_state(ScreenState::Selection),
            EditorState::DelPopup => _ = self.popups.pop(),
        };
    }
}

impl Default for Interface {
    fn default() -> Self {
        Self::new(TodoListCollection::default())
    }
}

pub enum InterfaceState {
    Quit(Result<(), String>),
}

impl EventHandler<(), InterfaceState> for Interface {
    fn handle_event(&mut self, event: &Event, _: ()) -> Option<InterfaceState> {
        self.handle_resize(event);
        if self.should_quit(event) {
            return Some(InterfaceState::Quit(Ok(())));
        }

        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => {
                    if self.help_screen.active {
                        self.help_screen.active = false;
                        self.change_state(ScreenState::Selection);
                    }
                }
                KeyCode::Char('?') => {
                    self.help_screen.active = !self.help_screen.active;
                    if self.help_screen.active {
                        self.change_state(ScreenState::Help);
                    } else {
                        self.change_state(ScreenState::Selection);
                    }
                }

                _ => {}
            };
        }

        match self.screen_state {
            ScreenState::Selection => self.handle_selection_bar(event),
            ScreenState::Editor => self.handle_editor(event),
            ScreenState::Help => {
                if let Event::Key(key) = event {
                    if key.code == KeyCode::Esc {
                        self.change_state(ScreenState::Selection);
                    }
                }
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
        let mut editor = Editor::default();
        editor.viewport = viewport;
        let mut selection_bar = SelectionBar::default();
        selection_bar.viewport = viewport;

        let mut names = vec![];
        // this clone isnt good, maybe passing Vec<&String> is better or doing something else
        collection.lists.iter().for_each(|list| names.push(list.title.clone()));
        //selection_bar.set_names(names);

        Self {
            popups: vec![],
            terminal,
            collection,
            editor,
            selection_bar,
            help_screen: HelpScreen::default(),

            selected_list: 0,
            screen_size: viewport,
            screen_state: ScreenState::default(),
        }
    }

    pub fn draw(&mut self) {
        let names = self.collection_names();
        _ = self.terminal.draw(|frame| {
            let layout = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)]);
            let [selection_area, editor_area] = layout.areas(frame.area());

            match self.screen_state {
                ScreenState::Selection => {
                    // have to do x+1, else it puts the cursor at | instead of [
                    let (y, x) = self.selection_bar.cursor_pos();
                    let position = Position::new(x + 1, y + 1);
                    frame.set_cursor_position(position);
                }

                ScreenState::Editor => {
                    let padding: u16 = 4; // padding is `[ ] `
                    let x = self.editor.cursor.x + 1;
                    let y = self.editor.cursor.y + 1;
                    let position = Position::new(editor_area.x + x + padding, y);
                    frame.set_cursor_position(position);
                }
                ScreenState::Help => {
                    self.help_screen.draw(frame.area(), frame);
                    return;
                }
            };

            let list = self.collection.lists.get(self.selected_list);
            self.selection_bar.draw(frame, selection_area, names);
            self.editor.draw(frame, editor_area, list);

            if let Some(popup) = self.popups.last() {
                match popup {
                    PopupState::Editor => self.editor.draw_popup(frame),
                    PopupState::Selection => self.selection_bar.draw_popup(frame),
                }
            }
        });
    }

    pub fn add_todo(&mut self, content: &str) {
        let list = self.collection.get_mut_todo_list(self.selected_list);
        if let Some(list) = list {
            list.push_str(content)
        }
    }

    pub fn remove_todo(&mut self) {
        let list = self.collection.get_mut_todo_list(self.selected_list);
        if let Some(list) = list {
            list.data.remove(self.editor.cursor.x as usize);
        }
    }

    pub fn deinit(&self) {
        restore();
    }

    pub fn get_editor_viewport(&self) -> &Viewport {
        &self.screen_size
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

    //pub fn set_selection_names(&mut self, names: Vec<String>) {
    //    self.selection_bar.set_names(names);
    //}

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
