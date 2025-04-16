use serde::{Deserialize, Serialize};
use std::io::Error;
use std::io::ErrorKind;
use time::OffsetDateTime;

// Base todo struct
#[serde_with::serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct Todo {
    #[serde_as(as = "time::format_description::well_known::Rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "Option<time::format_description::well_known::Rfc3339>")]
    pub completed_at: Option<OffsetDateTime>,
    pub title: String,
    pub completed: bool,
}

pub struct Todos {
    pub todo_list: Vec<Todo>,
}

// Implemented methods for a todo item.
impl Todos {
    pub fn new() -> Self {
        Self {
            todo_list: Vec::new(),
        }
    }
    pub fn add(&mut self, title: String) {
        let todo = Todo {
            created_at: time::OffsetDateTime::now_utc(),
            completed_at: None,
            title,
            completed: false,
        };

        self.todo_list.push(todo);
    }

    pub fn toggle(&mut self, index: usize) -> Result<String, Error> {
        if self.todo_list.len() < index {
            return Err(Error::new(ErrorKind::InvalidInput, "Index out of bounds"));
        }

        let status = !self.todo_list.get(index).unwrap().completed;
        self.todo_list.get_mut(index).unwrap().completed = status;
        self.todo_list.get_mut(index).unwrap().completed_at = Some(time::OffsetDateTime::now_utc());

        Ok(String::from("OK"))
    }

    pub fn delete(&mut self, index: usize) -> Result<String, Error> {
        if self.todo_list.len() < index {
            return Err(Error::new(ErrorKind::InvalidInput, "Index out of bounds"));
        }

        self.todo_list.remove(index);

        Ok(String::from("Ok"))
    }

    pub fn edit(&self, index: usize, title: String) {}

    pub fn list(&self) {
        for (i, val) in self.todo_list.iter().enumerate() {
            let mut completed_at = String::from("");
            if val.completed_at.is_some() {
                completed_at = val.completed_at.unwrap().to_string();
            }

            println!(
                "{i} {0}: Done: {1}, Create: {2} Completed: {3}",
                val.title,
                val.completed.to_string(),
                val.created_at.to_string(),
                completed_at
            );
        }
    }
}
