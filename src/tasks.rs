use chrono::{DateTime, Utc, Local, serde::ts_seconds};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs::{OpenOptions, File, remove_file};
use std::io::{Result, Seek, SeekFrom};
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>
}

impl Task {
    fn new(title: String) -> Task {
        Task { title: title, created_at: Utc::now() }
    }

    fn update(&mut self, title: String) {
        self.title = title
    }
}


pub fn add(file_path: PathBuf, title: String) -> Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).create(true).open(file_path)?;

    let mut tasks = match read_tasks(&mut file) {
        Ok(tasks) => tasks,
        Err(e) => Err(e)?
    };
    tasks.push(Task::new(title));
    serde_json::to_writer(file, &tasks)?;
    println!("Task added.");
    Ok(())
}

pub fn update(file_path: PathBuf, index: usize, title: String) -> Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).create(true).open(file_path)?;
    let mut tasks = match read_tasks(&mut file) {
        Ok(tasks) => tasks,
        Err(e) => Err(e)?
    };
    if index > tasks.len() || index <= 0 {
        println!("Invalid index");
        return Ok(());
    }
    tasks[index - 1].update(title);
    file.seek(SeekFrom::Start(0))?;
    serde_json::to_writer(file, &tasks)?;
    println!("Task updated.");
    Ok(())
}

pub fn delete(file_path: PathBuf, index: usize) -> Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).create(true).open(file_path)?;
    let mut tasks = match read_tasks(&mut file) {
        Ok(tasks) => tasks,
        Err(e) => Err(e)?
    };
    if index > tasks.len() || index <= 0 {
        println!("Invalid index");
        return Ok(());
    }
    tasks.remove(index - 1);
    file.seek(SeekFrom::Start(0))?;
    serde_json::to_writer(file, &tasks)?;
    println!("Task deleted.");
    Ok(())
}

pub fn delete_all(file_path: PathBuf) -> Result<()> {
    remove_file(file_path)?;
    println!("Tasks deleted.");
    Ok(())
}

fn get(file_path: PathBuf) -> Result<Vec<Task>> {
    let mut file = OpenOptions::new().create(true).write(true).read(true).open(file_path)?;
    let tasks = match read_tasks(&mut file) {
        Ok(tasks) => tasks,
        Err(e) => Err(e)?
    };
    Ok(tasks)
}

fn read_tasks(file: &mut File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;
    let tasks: Vec<Task> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?
    };
    Ok(tasks)
}


pub fn show_tasks(file_path: PathBuf) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let tasks = get(file_path)?;
    if tasks.len() == 0 {
        println!("No task available.");
        return Ok(())
    }
    let mut table_vector = Vec::new();
    for i in 0..tasks.len() {
        table_vector.push(
            vec![
                (i + 1).cell().justify(Justify::Center), 
                tasks[i].title.as_str().cell().justify(Justify::Center),
                tasks[i].created_at.with_timezone(&Local).cell().justify(Justify::Center)
            ]
        )
    }
    let table = table_vector.table().title(
        vec![
            "Index".cell().bold(true).justify(Justify::Center),
            "Task".cell().bold(true).justify(Justify::Center),
            "Created At".cell().bold(true).justify(Justify::Center)
        ]
    ).bold(true);
    assert!(print_stdout(table).is_ok());
    Ok(())
}

