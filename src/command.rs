use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "todo")]
#[command(about = "A simple todo CLI app", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Create a new todo item with a title
    Create {
        /// The title of the todo item
        title: String,
    },
    /// Delete a todo item by index
    Delete {
        /// The index of the item to delete
        index: usize,
    },
    /// Edit a todo item by index
    Edit {
        /// The index of the item to edit
        index: usize,
        /// The title of the todo item
        title: String,
    },
    /// Toggle the completion status of a todo item by index
    Toggle {
        /// The index of the item to toggle
        index: usize,
    },
    /// List all todo items
    List,
}

