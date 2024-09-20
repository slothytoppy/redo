use std::str::FromStr;

use crate::todo::{Todo, TodoList};

fn is_valid(start: &str) -> bool {
    // prevents panic on iteration
    if start.len() < 3 {
        return false;
    }
    let a = start.chars().nth(0).unwrap();
    let b = start.chars().nth(1).unwrap();
    let c = start.chars().nth(2).unwrap();
    a == '[' && c == ']' && matches!(b, ' ' | 'x')
}

pub fn parse<'list>(content: &'list str) -> Result<TodoList<'list>, String> {
    if content.len() == 0 {
        return Err("Could not parse because contents was empty".to_string());
    }
    let todos = content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter(|line| is_valid(line))
        .map(Into::into)
        .collect::<Vec<_>>();
    let list = TodoList { data: todos };
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
