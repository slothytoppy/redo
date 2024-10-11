use crossterm::event::read;
use redo::{filesystem, parser};

use crate::event::EventHandler;
use crate::tui::{Interface, InterfaceState};

#[derive(Debug, Default)]
pub struct App {
    pub file: String,
    interface: Interface,
}

impl App {
    pub fn init(args: std::env::Args) -> Self {
        ratatui::init();

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

        let content = filesystem::read(&file).unwrap_or_default();
        let collection = parser::parse_collection(&content).unwrap_or_default();

        let interface = Interface::new(collection);

        Self { file, interface }
    }

    pub fn run(&mut self) {
        loop {
            self.interface.draw();
            self.interface.flush();
            let event = read().unwrap();

            if let Some(InterfaceState::Quit(str)) = self.interface.handle_event(&event, ()) {
                self.deinit();
                if let Err(str) = str {
                    eprintln!("{}", str);
                    panic!();
                }
                break;
            }
        }
    }

    pub fn deinit(&self) {
        self.interface.deinit();

        let mut tmp = String::default();

        for list in &self.interface.collection.lists {
            tmp.push_str(&format!("{}:\n", list.title));
            for todos in &list.data {
                tmp.push_str(&format!("{} {}\n", todos.status, todos.data));
            }
        }

        if !filesystem::write(&self.file, tmp) {
            tracing::info!("failed to write to file {}", &self.file);
        }
    }
}
