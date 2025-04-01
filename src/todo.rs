use time::{UtcDateTime};

// Base todo struct
struct Todo {
    created_at: UtcDateTime,
    completed_at: UtcDateTime,
    title: String,
    completed: bool,
}

struct Todos{todo: Vec<Todo>}

// Implemented methods for a todo item.
impl Todos {
    pub fn new(&mut self, title: String) {
        let todo = Todo {
            created_at: time::UtcDateTime::now(),
            completed_at:time::UtcDateTime::now(),
            title,
            completed: false,
        };

        self.todo.push(todo);
    }
}
