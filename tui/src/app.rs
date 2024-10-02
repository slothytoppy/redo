use crossterm::cursor::MoveTo;
use crossterm::event::read;
use crossterm::style::Print;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use redo::todo::TodoListCollection;
use redo::{filesystem, parser, TodoList};

use crate::tui::Interface;
use crate::viewport::Viewport;

#[derive(Default, Debug)]
pub struct App {
    pub file: String,
    pub collection: TodoListCollection,
    viewport: Viewport,
    interface: Interface,
}

impl App {
    pub fn get_todo_list(&mut self, index: usize) -> Option<&mut TodoList> {
        match self.collection.get_mut_todo_list(index) {
            Some(list) => Some(list),
            None => None,
        }
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
        let collection = parser::parse_collection(&content).unwrap_or_default();

        let viewport = match crossterm::terminal::window_size() {
            Ok(size) => Viewport::new(size.height, size.rows),
            Err(..) => Viewport::default(),
        };

        Self {
            collection,
            file,
            viewport,
            ..Default::default()
        }
    }

    pub fn deinit(&self) {
        let _ = disable_raw_mode();
        let _ = crossterm::execute!(std::io::stdout(), LeaveAlternateScreen);
        let mut tmp = String::default();
        for list in &self.collection.lists {
            list.data.iter().for_each(|todo| tmp.push_str(&format!("{todo}")));
        }
        filesystem::write(&self.file, tmp);
    }

    pub fn run(&mut self) {
        let mut names = vec![];
        for list in &self.collection.lists {
            if let Some(name) = &list.name {
                names.push(name.to_string());
            };
        }

        let lists = &self.collection.lists;
        if !lists.is_empty() {
            let list = lists[0].clone();
            self.interface.update_editor_list(list);
        }

        self.interface.change_collection_names(names);
        let args = std::env::args();
        Self::init(args);

        self.interface.draw();
        self.interface.move_to();
        self.interface.flush();

        loop {
            let event = read().unwrap();

            self.interface.handle_event(&event);
            if self.interface.should_quit(&event) {
                break;
            }
            self.interface.flush();
        }
        self.deinit();
    }
}
