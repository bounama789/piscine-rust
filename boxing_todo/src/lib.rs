// lib.rs

mod err;
use err::{ParseErr, ReadErr};

pub use json::{parse, stringify};
pub use std::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let file_content = std::fs::read_to_string(path).map_err(|e| ReadErr {
            child_err: Box::new(e)
        })?;
        if file_content.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }
        
        match json::parse(&file_content) {
            Ok(value) => {
                let mut todo_list = TodoList {
                    title: value["title"].to_string(),
                    tasks: Vec::new(),
                };

                value["tasks"].members().for_each(|k| {
                    let id: u32 = k["id"].to_string().parse().unwrap();
                    let level: u32 = k["level"].to_string().parse().unwrap();
                    todo_list.tasks.push(Task {
                        id,
                        description: k["description"].to_string(),
                        level,
                    })
                });

                if todo_list.tasks.len() == 0 {
                    return Err(Box::new(ParseErr::Empty));
                }

                Ok(todo_list)
            }
            Err(e) => Err(Box::new(ParseErr::Malformed(Box::new(e)))),
        }
    }
}
