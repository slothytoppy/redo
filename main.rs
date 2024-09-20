mod app;
mod filesystem;
mod parser;
mod todo;
use todo::TodoList;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = &args[1];
    let contents = filesystem::read(file);
    let list = parser::parse(&contents).expect("full");
    println!("{}", parser::collect_todos(list))
    //list.serialize("./redo.todo");
    // i think i might do that? im not sure kek yeah but if it doesnt start with [ ] then it can
    // never be a todo, it can never be done at least
}
