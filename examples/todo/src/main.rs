use anyhow::{Context, Result};
use chrono::Local;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple CLI todo app", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo
    Add {
        /// Todo description
        description: String,
    },
    /// List all todos
    List,
    /// Mark a todo as done
    Done {
        /// Todo ID
        id: usize,
    },
    /// Delete a todo
    Delete {
        /// Todo ID
        id: usize,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: usize,
    description: String,
    done: bool,
    created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoStore {
    todos: Vec<Todo>,
    next_id: usize,
}

impl TodoStore {
    fn new() -> Self {
        TodoStore {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    fn load(path: &PathBuf) -> Result<Self> {
        if path.exists() {
            let data = fs::read_to_string(path)
                .with_context(|| format!("Failed to read {}", path.display()))?;
            let store: TodoStore =
                serde_json::from_str(&data).with_context(|| "Failed to parse todo data")?;
            Ok(store)
        } else {
            Ok(TodoStore::new())
        }
    }

    fn save(&self, path: &PathBuf) -> Result<()> {
        let data =
            serde_json::to_string_pretty(self).with_context(|| "Failed to serialize todo data")?;
        fs::write(path, data).with_context(|| format!("Failed to write {}", path.display()))?;
        Ok(())
    }

    fn add(&mut self, description: String) -> &Todo {
        let todo = Todo {
            id: self.next_id,
            description,
            done: false,
            created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        self.next_id += 1;
        self.todos.push(todo);
        self.todos.last().unwrap()
    }

    fn list(&self) {
        if self.todos.is_empty() {
            println!("No todos yet! Use 'todo add <description>' to add one.");
            return;
        }

        println!(
            "{:<5} {:<5} {:<30} {}",
            "ID", "Done", "Description", "Created At"
        );
        println!("{}", "-".repeat(75));
        for todo in &self.todos {
            let done_str = if todo.done { "✅" } else { "⬜" };
            println!(
                "{:<5} {:<5} {:<30} {}",
                todo.id, done_str, todo.description, todo.created_at
            );
        }
    }

    fn mark_done(&mut self, id: usize) -> Result<()> {
        let todo = self
            .todos
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or_else(|| anyhow::anyhow!("Todo with ID {} not found", id))?;
        todo.done = true;
        Ok(())
    }

    fn delete(&mut self, id: usize) -> Result<Todo> {
        let pos = self
            .todos
            .iter()
            .position(|t| t.id == id)
            .ok_or_else(|| anyhow::anyhow!("Todo with ID {} not found", id))?;
        Ok(self.todos.remove(pos))
    }
}

fn get_data_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".todo-cli-data.json");
    path
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let data_path = get_data_path();
    let mut store = TodoStore::load(&data_path)?;

    match cli.command {
        Commands::Add { description } => {
            let todo = store.add(description);
            println!("Added todo #{}: {}", todo.id, todo.description);
            store.save(&data_path)?;
        }
        Commands::List => {
            store.list();
        }
        Commands::Done { id } => {
            store
                .mark_done(id)
                .with_context(|| format!("Failed to mark todo {} as done", id))?;
            println!("Marked todo #{} as done", id);
            store.save(&data_path)?;
        }
        Commands::Delete { id } => {
            let todo = store
                .delete(id)
                .with_context(|| format!("Failed to delete todo {}", id))?;
            println!("Deleted todo #{}: {}", todo.id, todo.description);
            store.save(&data_path)?;
        }
    }

    Ok(())
}
