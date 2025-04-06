use std::io;

use todo::Todos;

mod todo;

fn main() {
    println!("Welcome to the RUSTY TODO-LIST");
    // TODO: Add storage loading here later...
    let mut todos = todo::Todos::new();

    loop {
        println!("What do you wanna do?");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        // TODO: Replace this with options handling
        match choice.as_str().trim() {
            "create" => create_opt(String::from("Test"), &mut todos) ,
            "edit" => println!("Here is the EDIT TODO function"),
            "delete" => println!("Here is the DELETE TODO function"),
            "toggle" => println!("Here is the TOGGLE TODO function"),
            "list" => todos.list(),
            _ => println!(
                "Try using the following: create, edit, delete,
                toggle, list"
            ),
        };
    }
}

fn create_opt(title : String,  t : &mut todo::Todos) {
    t.add(title);
}
