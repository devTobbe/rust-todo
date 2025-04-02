use std::io;

use time::UtcDateTime;

// Base todo struct
pub struct Todo {
    created_at: UtcDateTime,
    completed_at: UtcDateTime,
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
            completed_at: time::UtcDateTime::now(),
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
}
