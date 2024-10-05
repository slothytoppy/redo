use crossterm::event::read;
use redo::{filesystem, parser};

use crate::event::EventHandler;
use crate::tui::Interface;
use crate::viewport::Viewport;

#[derive(Debug, Default)]
pub struct App {
    pub file: String,
    viewport: Viewport,
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

        let content = filesystem::read(&file);
        let collection = parser::parse_collection(&content).unwrap_or_default();
        let interface = Interface::new(collection);

        let viewport = match crossterm::terminal::window_size() {
            Ok(size) => Viewport::new(size.columns, size.rows),
            Err(_) => panic!("can not request window_size"),
        };

        Self {
            file,
            viewport,
            interface,
        }
    }

    pub fn deinit(&self) {
        self.interface.deinit();
        let mut tmp = String::default();
        for list in &self.interface.collection.lists {
            list.data.iter().for_each(|todo| tmp.push_str(&format!("{todo}")));
        }
        filesystem::write(&self.file, tmp);
    }

    pub fn run(&mut self) {
        let args = std::env::args();
        Self::init(args);

        let mut names = vec![];
        for list in &self.interface.collection.lists {
            if let Some(name) = &list.name {
                names.push(name.to_string());
            };
        }

        self.interface.change_collection_names(names);

        let mut longest_name = 0;
        for name in self.interface.collection_names() {
            if name.len() as u16 > longest_name {
                longest_name = name.len() as u16
            }
        }

        self.interface.set_editor_viewport();

        self.interface.draw();
        self.interface.flush();

        loop {
            let event = read().unwrap();

            if let Some(true) = self.interface.handle_event(&event, &()) {
                break;
            }

            self.interface.draw();
            self.interface.flush();
        }
        self.deinit();
    }
}
