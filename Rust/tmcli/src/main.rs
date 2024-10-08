use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    title: String,
    description: String,
    completed: bool,
}

impl Task {
    fn new(title: String, description: String) -> Self {
        Task {
            title,
            description,
            completed: false, // Default to incomplete
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }

    fn mark_incomplete(&mut self) {
        self.completed = false;
    }

    fn to_dict(&self) -> HashMap<String, String> {
        let mut task_hmap = HashMap::new();
        task_hmap.insert(String::from("title"), self.title.clone());
        task_hmap.insert(String::from("description"), self.description.clone());
        task_hmap.insert(String::from("completed"), self.completed.to_string());
        task_hmap
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = if self.completed { "✓" } else { "✗" };
        write!(f, "[{}] {}: {}", status, self.title, self.description)
    }
}

struct TaskManager {
    storage_file: String,
    tasks: Vec<Task>,
}

impl TaskManager {
    fn new(storage_file: &str) -> Self {
        TaskManager {
            storage_file: String::from(storage_file),
            tasks: TaskManager::load_tasks(storage_file).unwrap_or_else(|_| vec![]),
        }
    }

    fn load_tasks(storage_file: &str) -> io::Result<Vec<Task>> {
        if Path::new(storage_file).exists() {
            let mut file = File::open(storage_file)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let tasks: Vec<Task> = serde_json::from_str(&contents)?;
            Ok(tasks)
        } else {
            Ok(vec![])
        }
    }

    fn save_tasks(&self) -> io::Result<()> {
        let mut file = File::create(&self.storage_file)?;
        let tasks_data: Vec<HashMap<String, String>> = self.tasks.iter().map(|task| task.to_dict()).collect();
        serde_json::to_writer(&mut file, &tasks_data)?;
        Ok(())
    }

    fn create_task(&mut self) {
        let mut title = String::new();
        let mut description = String::new();

        print!("What is the name of the task?: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut title).unwrap();
        title = title.trim().to_string();

        print!("Would you like to put in a description of the task? [Enter to cancel]: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut description).unwrap();
        description = description.trim().to_string();

        if !title.is_empty() {
            let new_task = Task::new(title, description);
            self.tasks.push(new_task);
            self.save_tasks().unwrap();
        }
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks available.");
        } else {
            for task in &self.tasks {
                println!("{}", task);
            }
        }
    }

    fn delete_task(&mut self) {
        self.list_tasks();

        let mut title = String::new();
        print!("Enter the title of the task you want to delete: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut title).unwrap();
        title = title.trim().to_string();

        if let Some(pos) = self.tasks.iter().position(|task| task.title.eq_ignore_ascii_case(&title)) {
            let task_to_delete = self.tasks.remove(pos);
            self.save_tasks().unwrap();
            println!("Task \"{}\" has been deleted.", task_to_delete.title);
        } else {
            println!("Task \"{}\" not found.", title);
        }
    }
}

fn main() {
    let mut task_manager = TaskManager::new("tasks.json");

    loop {
        println!("\nTask Manager");
        println!("1. Create Task");
        println!("2. List Tasks");
        println!("3. Delete Task");
        println!("4. Exit");
        let mut choice = String::new();
        print!("Choose an option: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => task_manager.create_task(),
            "2" => task_manager.list_tasks(),
            "3" => task_manager.delete_task(),
            "4" => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
