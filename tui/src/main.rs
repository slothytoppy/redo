use crossterm;
use redo::TodoList;

struct App<'app> {
    data: TodoList<'app>,
}

impl<'app> App<'app> {}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = &args[1];
    let contents = redo::filesystem::read(file);
    let list = redo::parser::parse(&contents).expect("full");
    println!("{}", redo::parser::collect_todos(list));
    // i think i might do that? im not sure kek yeah but if it doesnt start with [ ] then it can
    // never be a todo, it can never be done at least
}
