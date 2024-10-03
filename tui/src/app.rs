use crossterm::cursor::MoveTo;
use crossterm::event::read;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use redo::{filesystem, parser};

use crate::event::EventHandler;
use crate::tui::Interface;
use crate::viewport::Viewport;

#[derive(Default, Debug)]
pub struct App {
    pub file: String,
    viewport: Viewport,
    interface: Interface,
}

impl App {
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
        let collection = parser::parse_collection(&content).unwrap_or_default();
        let interface = Interface::new(collection);

        let viewport = match crossterm::terminal::window_size() {
            Ok(size) => Viewport::new(size.columns, size.rows),
            Err(_) => Viewport::default(),
        };

        Self {
            file,
            viewport,
            interface,
        }
    }

    pub fn deinit(&self) {
        let _ = disable_raw_mode();
        let _ = crossterm::execute!(std::io::stdout(), LeaveAlternateScreen);
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

        let lists = &self.interface.collection.lists;
        if !lists.is_empty() {
            let list = lists[0].clone();
            self.interface.update_editor_list(list);
        }

        let mut longest_name = 0;
        for name in self.interface.collection_names() {
            if name.len() as u16 > longest_name {
                longest_name += name.len() as u16
            }
        }

        self.interface
            .set_selection_viewport(Viewport::new(self.viewport.y(), longest_name));
        self.interface
            .set_editor_viewport(Viewport::new(self.viewport.y(), longest_name + 1));

        self.interface.draw();
        self.interface.flush();

        loop {
            let event = read().unwrap();

            if let Some(true) = self.interface.handle_event(&event) {
                break;
            }

            self.interface.draw();
            self.interface.flush();
        }
        self.deinit();
    }
}
