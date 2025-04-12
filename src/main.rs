mod command;
mod todo;

use clap::Parser;
use command::{Cli, Command};

fn main() {
    println!("Welcome to the RUSTY TODO-LIST");
    // TODO: Add storage loading here later...
    let mut todos = todo::Todos::new();

    let cli = Cli::parse();

    match cli.command {
        Command::Create { title } => {
            todos.add(title);
        }
        Command::Delete { index } => {
            let _ = todos.delete(index);
        }
        Command::Toggle { index } => {
            let _ = todos.toggle(index);
        }
        Command::Edit { index, title }  => {
            let _ = todos.edit(index, title);
        }
        Command::List => {
            todos.list()
        }
    }
}
