use std::io;

use time::UtcDateTime;

// Base todo struct
pub struct Todo {
    created_at: UtcDateTime,
    completed_at: Option<UtcDateTime>,
    title: String,
    completed: bool,
}

pub struct Todos {
    todo_list: Vec<Todo>,
}

// Implemented methods for a todo item.
impl Todos {
    pub fn new() -> Self {
        Self {todo_list: Vec::new()}
    }
    pub fn add(&mut self, title: String) {
        let todo = Todo {
            created_at: time::UtcDateTime::now(),
            completed_at: None,
            title,
            completed: false,
        };

        self.todo_list.push(todo);
    }

    pub fn delete(&mut self, index: usize) -> Result<String, io::Error> {
        if self.todo_list.len() > index {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Index out of bounds",
            ));
        }

        let title = self.todo_list.remove(index).title;

        Ok((&title).to_string())
    }

    pub fn list(&self) {
        for (i, val) in self.todo_list.iter().enumerate() {
            let mut completed_at = String::from("");
            if val.completed_at.is_some(){
                completed_at = val.completed_at.unwrap().to_string();
            }

            println!("{i} {0}: Done: {1}, Create: {2} Completed: {3}", val.title, val.completed.to_string(), val.created_at.to_string(), completed_at);
        }

    }
}
