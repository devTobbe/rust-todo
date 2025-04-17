mod command;
mod storage;
mod todo;

use clap::Parser;
use command::{Cli, Command};

fn main() {
    // Read from storage file
    let mut todos = storage::readfile();
    let cli = Cli::parse();

    // Handle cli input flags/options
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
    // Write to storage file
    storage::writefile(&todos);
}
