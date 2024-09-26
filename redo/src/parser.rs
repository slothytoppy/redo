use crate::todo::{Todo, TodoList, TodoStatus};

fn is_valid(start: &str) -> bool {
    // prevents panic on iteration
    if start.len() < 3 {
        return false;
    }
    let a = start.chars().next().unwrap();
    let b = start.chars().nth(1).unwrap();
    let c = start.chars().nth(2).unwrap();
    a == '[' && c == ']' && matches!(b, ' ' | 'x')
}

impl From<String> for Todo {
    fn from(value: String) -> Self {
        let status = match value.starts_with("[x]") {
            true => TodoStatus::Complete,
            false => TodoStatus::Incomplete,
        };
        Todo { status, data: value }
    }
}

pub fn parse(content: &str) -> Result<TodoList, String> {
    if content.is_empty() || !is_valid(content) {
        return Err("Could not parse because contents was empty".to_string());
    }
    let data = content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter(|line| is_valid(line))
        .map(|line| line[3..].trim().to_string())
        .map(Into::into)
        .collect::<Vec<_>>();

    let list = TodoList { data };
    Ok(list)
}

pub fn collect_todos(contents: TodoList) -> String {
    contents
        .data
        .iter()
        .map(|todo| todo.data.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}
