use std::io;

mod todo;

fn main() {
    println!("Welcome to the RUSTY TODO-LIST");
    loop {
        println!("What do you wanna do?");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.as_str() {
            "create" => println!("Here is the CREATE TODO function"),
            "edit" => println!("Here is the EDIT TODO function"),
            "delete" => println!("Here is the DELETE TODO function"),
            "toggle" => println!("Here is the TOGGLE TODO function"),
            "list" => println!("Here is the LIST TODO function"),
            _ => println!(
                "Try using the following: create, edit, delete,
                toggle, list"
            ),
        };
    }
}

fn delete(index: u32) {
}

fn edit(index: u32, title: String) {
}

fn toggle(index: u32) {
}

fn list() {
}
