use std::{fs, io::Read, ops::Index};

#[derive(Debug, Default)]
struct Todo<'todo> {
    data: &'todo str,
    done: bool,
}

#[derive(Debug, Default)]
pub struct TodoList<'todo> {
    data: Vec<Todo<'todo>>,
}

impl<'todo> From<&'todo str> for Todo<'todo> {
    fn from(value: &'todo str) -> Self {
        Todo {
            data: value,
            done: value.starts_with("[x]"),
        }
    }
}

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

impl<'todo> TodoList<'todo> {
    // should this return an option or result or even just a bool? just to say hey this failed or
    // succeeded idk cuz this is gonna be like an app not a lib right? so its just internal stuff
    pub fn serialize<P: AsRef<std::path::Path>>(&self, file: P) {
        let mut data = String::default();
        self.data.iter().for_each(|line| data.push_str(line.data));
        fs::write(file, &data).expect("smh");
    }

    fn new(data: Vec<Todo<'todo>>) -> Self {
        Self { data }
    }

    pub fn parse(data: &'todo str) -> Self {
        let todos = data
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .filter(|line| is_valid(line))
            .map(Into::into)
            .collect::<Vec<_>>();
        println!("{todos:?}");
        Self::new(todos)
    }

    // i think just one file with todos inside
    // its creating todos
    pub fn deserialize<P: AsRef<std::path::Path>>(file: P) -> String {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(file.as_ref())
            .expect("cant open the file");

        let mut data = String::default();
        file.read_to_string(&mut data)
            .expect("file exists but it could not be read");
        data
    }
}

#[cfg(test)]
mod test {
    // smh i forget

    use super::*;

    #[test]
    fn test_deserialize() {
        let data = r#"
        [x] asdwasd
        [x] wehadhkjs
        [ ] urmom
        [ 
        "#;

        let res = TodoList::parse(data);
        println!("{res:?}");
        assert!(res.data.len() > 0);
        panic!();
    }
}
