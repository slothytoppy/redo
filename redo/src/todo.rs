use std::ops::{Index, IndexMut};

use crate::parser;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum TodoStatus {
    #[default]
    Incomplete,
    Complete,
}

impl From<bool> for TodoStatus {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Complete,
            false => Self::Incomplete,
        }
    }
}

impl TodoStatus {
    pub fn toggle(&mut self) {
        match self {
            Self::Complete => *self = Self::Incomplete,
            Self::Incomplete => *self = Self::Complete,
        }
    }
}

impl std::fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let done = match self {
            TodoStatus::Complete => "[x]",
            TodoStatus::Incomplete => "[ ]",
        };
        write!(f, "{}", done)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Todo {
    pub data: String,
    pub status: TodoStatus,
}

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.status, self.data)
    }
}

impl Todo {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[derive(Debug, Default, Clone)]
pub struct TodoList {
    pub name: Option<String>,
    pub data: Vec<Todo>,
}

impl TodoList {
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn len_line(&self, index: usize) -> usize {
        self.data[index].len()
    }

    pub fn new(name: Option<String>, contents: &str) -> Self {
        match parser::parse(contents) {
            Ok(list) => TodoList { data: list.data, name },
            Err(..) => TodoList { name, data: vec![] },
        }
    }

    pub fn push_str(&mut self, contents: &str) {
        let todo = Todo {
            data: contents.to_string(),
            status: TodoStatus::Incomplete,
        };
        self.data.push(todo);
    }

    pub fn push_todo(&mut self, todo: Todo) {
        self.data.push(todo);
    }

    pub fn pop(&mut self) {
        self.data.pop();
    }
}

impl std::fmt::Display for TodoList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut data = String::default();
        self.data
            .iter()
            .for_each(|todo| data.push_str(&format!("{} {}", todo.status, todo.data)));
        write!(f, "{}", data)
    }
}

#[derive(Debug, Default, Clone)]
pub struct TodoListCollection {
    pub lists: Vec<TodoList>,
}

impl TodoListCollection {
    pub fn push(&mut self, list: TodoList) {
        self.lists.push(list);
    }

    pub fn get_todo_list(&self, index: usize) -> Option<&TodoList> {
        if self.lists.is_empty() || index > self.lists.len() {
            return None;
        }
        let list = self.lists.index(index);
        Some(list)
    }

    pub fn get_mut_todo_list(&mut self, index: usize) -> Option<&mut TodoList> {
        if self.lists.is_empty() || index > self.lists.len() {
            return None;
        }
        let list = self.lists.index_mut(index);
        Some(list)
    }
}

impl std::fmt::Display for TodoListCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.lists)
    }
}

#[cfg(test)]
mod test {
    use crate::parser;

    #[test]
    fn test_deserialize() {
        let data = r#"[x] asdwasd
        [x] wehadhkjs
        [ ] urmom
        [ ]
        "#;

        let res = parser::parse(data).expect("");
        assert!(!res.data.is_empty());
    }
}
