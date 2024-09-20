mod todo;
use todo::TodoList;
mod tui;

fn main() {
    let list = TodoList::deserialize("./main.todo");
    let list = TodoList::parse(&list);
    println!("{list:?}");
    list.serialize("./redo.todo");
    // i think i might do that? im not sure kek yeah but if it doesnt start with [ ] then it can
    // never be a todo, it can never be done at least
}
