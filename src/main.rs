mod cli;
mod tasks;

use dirs::data_local_dir;
use std::fs::create_dir_all;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Cli::get_arguments();
    let mut local_dir = match data_local_dir() {
        Some(directory) => directory,
        None => panic!("Unable to get local data directory: ")
    };
    local_dir = local_dir.join("todo_manager");
    create_dir_all(&local_dir)?;
    let file_path = local_dir.join("tasks.json");
    
    match args {
        cli::Cli::Add {title} => tasks::add(file_path, title)?,
        cli::Cli::Delete {index} => tasks::delete(file_path, index)?,
        cli::Cli::DeleteAll => tasks::delete_all(file_path)?,
        cli::Cli::Update {index, title} => tasks::update(file_path, index, title)?,
        cli::Cli::List => tasks::show_tasks(file_path)?
    }
    Ok(())
}
