use std::{fs::File, path::Path};

use serde_json::from_str;

use crate::todo::Todos;
use crate::todo::Todo;


/// Reads from the storage file todos.json.
/// Returns Todos object parsed from file. 
pub fn readfile() -> Todos {
    let mut todos : Todos = Todos::new();

    if !Path::new("todos.json").exists() {
        return todos;
    };

    let file = std::fs::read_to_string("todos.json").unwrap();
    todos.todo_list = from_str::<Vec<Todo>>(&file).unwrap();

    return todos;
}

/// Writes to the storage file todos.json
pub fn writefile(todos : &Todos){

    if !Path::new("todos.json").exists() {
        let _ = File::create("todos.json");
    };

    let json = serde_json::to_string_pretty(&todos.todo_list).unwrap();
    let _ = std::fs::write("todos.json", json);
}
