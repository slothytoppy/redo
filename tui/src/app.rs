use std::io::{self, Write};
use std::ops::IndexMut;

use crossterm::cursor::{MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp};
use crossterm::event::Event::Key;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{cursor, event, execute, queue};
use redo::{filesystem, parser, TodoList};

use crate::tui::{self, Screen};

#[derive(Default, Debug)]
pub struct TodoListCollection {
    pub list: Vec<TodoList>,
}

impl TodoListCollection {
    pub fn get_todo_list(&mut self, index: usize) -> &mut TodoList {
        self.list.index_mut(index)
    }
}

#[derive(Default, Debug)]
pub struct App {
    pub list: TodoListCollection,
    pub file: String,
    renderer: tui::Renderer,
    screen: Screen,
}

impl App {
    pub fn get_todo_list(&mut self, index: usize) -> &mut TodoList {
        self.list.get_todo_list(index)
    }

    pub fn init(args: std::env::Args) -> Self {
        let _ = enable_raw_mode();
        let _ = crossterm::execute!(
            std::io::stdout(),
            EnterAlternateScreen,
            Clear(ClearType::All),
            MoveTo(0, 0)
        );
        if args.len() <= 1 {
            return App::default();
        }
        let mut args = args.skip(1);
        let file = match args.next() {
            Some(data) => data,
            None => {
                return App::default();
            }
        };
        let content = filesystem::read(&file);
        let todos = TodoListCollection {
            list: vec![parser::parse(&content).unwrap_or_default()],
        };
        Self {
            list: todos,
            file,
            ..Default::default()
        }
    }

    pub fn deinit(&self) {
        let _ = disable_raw_mode();
        let _ = crossterm::execute!(std::io::stdout(), LeaveAlternateScreen);
        let mut tmp = String::default();
        for list in &self.list.list {
            list.data.iter().for_each(|todo| tmp.push_str(&format!("{todo}")));
        }
        filesystem::write(&self.file, tmp);
    }

    pub fn run(&mut self) {
        let args = std::env::args();
        Self::init(args);
        let key_quit = Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        });

        let mut buffer = String::default();
        let mut stdout = io::stdout();
        self.renderer.draw(&self.screen, &self.list);
        let _ = stdout.flush();

        loop {
            let event = read().unwrap();
            match event {
                Event::Key(key) => {
                    match get_direction(key.code) {
                        Some(Direction::Up) => {
                            let _ = queue!(io::stdout(), MoveUp(1));
                        }
                        Some(Direction::Down) => {
                            let _ = queue!(io::stdout(), MoveDown(1));
                        }
                        Some(Direction::Left) => {
                            let _ = queue!(io::stdout(), MoveLeft(1));
                        }
                        Some(Direction::Right) => {
                            let _ = queue!(io::stdout(), MoveRight(1));
                        }
                        _ => {}
                    }
                    if event == key_quit {
                        break;
                    }
                    if let KeyCode::Char(char) = key.code {
                        buffer.push(char);
                    }
                    if let KeyCode::Enter = key.code {
                        self.list.get_todo_list(0).push_str(&buffer);
                        buffer.clear();
                    }
                }
                _ => {}
            };
            self.renderer.draw(&self.screen, &self.list);
            let _ = stdout.flush();
        }
        self.deinit();
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_direction(key: KeyCode) -> Option<Direction> {
    match key {
        KeyCode::Up => Some(Direction::Up),
        KeyCode::Down => Some(Direction::Down),
        KeyCode::Left => Some(Direction::Left),
        KeyCode::Right => Some(Direction::Right),
        _ => None,
    }
}
