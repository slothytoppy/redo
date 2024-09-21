#[derive(Debug, Default)]
pub struct Todo<'todo> {
    pub data: &'todo str,
    done: bool,
}

#[derive(Debug, Default)]
pub struct TodoList<'todo> {
    pub data: Vec<Todo<'todo>>,
}

impl<'todo> From<&'todo str> for Todo<'todo> {
    fn from(value: &'todo str) -> Self {
        Todo {
            data: value,
            done: value.starts_with("[x]"),
        }
    }
}

impl<'todo> TodoList<'todo> {
    pub fn push(&mut self, contents: &'todo str) {
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

#[cfg(test)]
mod test {
    // smh i forget

    use super::*;
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
        assert!(res.data.len() > 0);
    }
}
