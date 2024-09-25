#[derive(Debug, Default)]
pub struct Todo {
    pub data: String,
    pub done: bool,
}

#[derive(Debug, Default)]
pub struct TodoList {
    pub data: Vec<Todo>,
}

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let done = match self.done {
            true => "[x]",
            false => "[ ]",
        };
        write!(f, "{} {}", done, self.data)
    }
}

impl TodoList {
    pub fn push_str(&mut self, contents: &str) {
        let todo = Todo {
            data: contents.to_string(),
            done: false,
        };
        self.data.push(todo);
    }

    pub fn push(&mut self, contents: String) {
        let todo = Todo {
            data: contents,
            done: false,
        };
        self.data.push(todo);
    }

    pub fn pop(&mut self) {
        self.data.pop();
    }
}

impl std::fmt::Display for TodoList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.data.is_empty() {
            return write!(f, "TodoList: empty");
        }
        let mut data = String::from("\n");
        self.data
            .iter()
            .for_each(|todo| data.push_str(&format!("{} {}\n", todo.data, todo.done).to_string()));
        write!(f, "TodoList: {}", data)
    }
}

#[cfg(test)]
mod test {
    use crate::parser;

    #[test]
    fn test_deserialize() {
        let data = r#"
        [x] asdwasd
        [x] wehadhkjs
        [ ] urmom
        [ 
        "#;

        let res = parser::parse(data).expect("TodoList was empty for some reason");
        assert!(res.data.is_empty());
    }
}
