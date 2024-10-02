use std::ops::IndexMut;

use crate::todo::{Todo, TodoList, TodoListCollection, TodoStatus};

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
    fn from(data: String) -> Self {
        let status = match data.starts_with("[x]") {
            true => TodoStatus::Complete,
            false => TodoStatus::Incomplete,
        };
        Todo { status, data }
    }
}

pub fn parse_todo(line: &str) -> Option<Todo> {
    if line.is_empty() || !is_valid(line) {
        return None;
    }
    let mut todo = Todo::default();
    if line.chars().nth(1).unwrap_or_default() == 'x' {
        todo.status = TodoStatus::Complete;
    } else {
        todo.status = TodoStatus::Incomplete;
    }
    todo.data = line[3..].trim().to_string();
    Some(todo)
}

pub fn parse(content: &str) -> Result<TodoList, String> {
    if content.is_empty() {
        return Err("Could not parse because contents was empty".to_string());
    }
    if !is_valid(content) {
        return Err(format!("Could not parse because contents was invalid: {content}"));
    }
    let data = content
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| is_valid(line))
        .map(|line| line[3..].trim().to_string())
        .map(Into::into)
        .collect::<Vec<_>>();

    let list = TodoList { name: None, data };
    Ok(list)
}

fn is_collection(line: &str) -> bool {
    if line.len() > 3 && line.starts_with("[") && line.ends_with("]:") {
        return true;
    }
    false
}

pub fn parse_collection(content: &str) -> Result<TodoListCollection, String> {
    if content.is_empty() {
        return Err("Could not parse because contents was empty".to_string());
    }

    let lines: Vec<&str> = content.split("\n").collect();
    let mut collection = TodoListCollection::default();
    let mut current_collection: usize = 0;

    for line in lines {
        let line = line.trim();
        match is_collection(line) {
            true => {
                let collection_name = line.trim_matches(':');
                collection.push(TodoList::new(Some(collection_name.to_string()), line));
                current_collection += 1;
            }
            false => {
                let list = &mut collection.lists.index_mut(current_collection.saturating_sub(1));
                if let Some(todo) = parse_todo(line) {
                    list.push_todo(todo)
                }
            }
        }
    }

    Ok(collection)
}

#[cfg(test)]
mod test {
    use super::parse_collection;

    #[test]
    fn parse_collection_test() {
        let content = r#"[workouts]:
        [ ] urmom"#;
        let collection = parse_collection(content).expect("");
        assert!(collection.lists.len() == 1);
        //panic!();
    }
}
