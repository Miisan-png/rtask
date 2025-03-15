use colored::*;
use dialoguer::{Confirm, Input, Select};
use std::fs;
use std::io;
use std::path::Path;
use chrono::{Local, NaiveDate};

use crate::model::{Task, SubTask, load_config, get_tasks_file, save_config};

pub fn load_tasks() -> Vec<Task> {
    let path = get_tasks_file();
    
    if !path.exists() {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                let _ = fs::create_dir_all(parent);
            }
        }
        return Vec::new();
    }
    
    match fs::read_to_string(&path) {
        Ok(data) => match serde_json::from_str(&data) {
            Ok(tasks) => tasks,
            Err(_) => Vec::new(),
        },
        Err(_) => Vec::new(),
    }
}

pub fn save_tasks(tasks: &[Task]) -> io::Result<()> {
    let path = get_tasks_file();
    
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    
    let json = serde_json::to_string_pretty(&tasks)?;
    fs::write(path, json)
}

pub fn add_task(name: String, priority: String, due: Option<String>, tags: Option<String>) {
    let mut tasks = load_tasks();
    let due_date = if let Some(due_str) = due {
        match NaiveDate::parse_from_str(&due_str, "%Y-%m-%d") {
            Ok(_) => Some(due_str),
            Err(_) => {
                println!("{}", "Invalid date format. Use YYYY-MM-DD".red().bold());
                return;
            }
        }
    } else {
        None
    };
    let tags_vec = tags
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();
    let now = chrono::Local::now();
    let task = Task {
        id: tasks.len() + 1,
        name,
        priority: priority.to_lowercase(),
        status: "pending".to_string(),
        progress: 0,
        due_date,
        tags: tags_vec,
        created_at: now.format("%Y-%m-%d %H:%M").to_string(),
        completed_at: None,
        subtasks: Vec::new(),
    };
    
    tasks.push(task.clone());
    
    match save_tasks(&tasks) {
        Ok(_) => {
            println!(
                "{} {}",
                "✓ Task added:".green().bold(),
                task.name.bright_white()
            );
        }
        Err(e) => {
            println!("{} {}", "Error adding task:".red().bold(), e);
        }
    }
}

pub fn format_priority(priority: &str) -> String {
    match priority.to_lowercase().as_str() {
        "high" => priority.bright_red().to_string(),
        "medium" => priority.yellow().to_string(),
        "low" => priority.green().to_string(),
        _ => priority.normal().to_string(),
    }
}

pub fn format_status(status: &str) -> String {
    match status.to_lowercase().as_str() {
        "done" => status.bright_green().to_string(),
        "pending" => status.yellow().to_string(),
        _ => status.normal().to_string(),
    }
}

pub fn format_progress_bar(progress: u8) -> String {
    let progress = progress.min(100);
    let filled = (progress as f32 / 10.0).ceil() as usize;
    let empty = 10 - filled;
    
    let filled_chars = "█".repeat(filled);
    let empty_chars = "░".repeat(empty);
    
    format!("[{}{}] {}%", filled_chars.green(), empty_chars, progress)
}

pub fn list_tasks(filter: Option<String>, all: bool, completed: bool) {
    let tasks = load_tasks();
    if tasks.is_empty() {
        println!("{}", "No tasks found".yellow());
        return;
    }
    let filtered_tasks: Vec<Task> = tasks
        .into_iter()
        .filter(|task| {
            let status_match = if completed {
                task.status == "done"
            } else if all {
                true
            } else {
                task.status != "done"
            };
            
            let tag_match = if let Some(tag_filter) = &filter {
                task.tags.iter().any(|tag| tag.contains(tag_filter))
            } else {
                true
            };
            
            status_match && tag_match
        })
        .collect();
    
    if filtered_tasks.is_empty() {
        println!("{}", "No matching tasks found".yellow());
        return;
    }
    let colored_tasks: Vec<Task> = filtered_tasks
        .iter()
        .map(|task| {
            let mut colored_task = task.clone();
            colored_task.priority = format_priority(&task.priority);
            colored_task.status = format_status(&task.status);
            colored_task
        })
        .collect();
    println!();
    for task in &colored_tasks {
        let id_str = format!("[{}]", task.id).cyan().bold();
        let priority_str = format!("[{}]", task.priority);
        let status_str = format!("[{}]", task.status);
        let progress_str = format_progress_bar(task.progress);
        
        let due_str = if let Some(due) = &task.due_date {
            format!("(Due: {})", due).yellow()
        } else {
            "".normal()
        };
        
        let tags_str = if !task.tags.is_empty() {
            format!("#{}",  task.tags.join(" #")).bright_blue()
        } else {
            "".normal()
        };
        
        let subtasks_str = if !task.subtasks.is_empty() {
            let completed = task.subtasks.iter().filter(|s| s.completed).count();
            format!("[{}/{}]", completed, task.subtasks.len()).bright_magenta()
        } else {
            "".normal()
        };
        
        println!("{} {} {} {} {} {} {} {}", 
            id_str,
            priority_str,
            status_str,
            progress_str,
            task.name.bright_white(),
            due_str,
            tags_str,
            subtasks_str
        );
    }
    println!();
    let pending_count = filtered_tasks
        .iter()
        .filter(|t| t.status == "pending")
        .count();
    
    let done_count = filtered_tasks.iter().filter(|t| t.status == "done").count();
    
    println!(
        "\n{} {} {}",
        "Summary:".cyan().bold(),
        format!("{} pending", pending_count).yellow(),
        format!("{} completed", done_count).green()
    );
}

pub fn complete_task(id: usize) {
    let mut tasks = load_tasks();
    
    let task_idx = tasks.iter().position(|t| t.id == id);
    
    match task_idx {
        Some(idx) => {
            if tasks[idx].status == "done" {
                println!("{}", "Task is already completed".yellow());
                return;
            }
            tasks[idx].status = "done".to_string();
            tasks[idx].progress = 100;
            tasks[idx].completed_at = Some(Local::now().format("%Y-%m-%d %H:%M").to_string());
            
            for subtask in &mut tasks[idx].subtasks {
                subtask.completed = true;
            }
            
            match save_tasks(&tasks) {
                Ok(_) => {
                    println!(
                        "{} {}",
                        "✓ Completed task:".green().bold(),
                        tasks[idx].name.bright_white()
                    );
                }
                Err(e) => {
                    println!("{} {}", "Error completing task:".red().bold(), e);
                }
            }
        }
        None => {
            println!("{}", format!("Task with ID {} not found", id).red());
        }
    }
}

pub fn remove_task(id: usize) {
    let mut tasks = load_tasks();
    
    let task_idx = tasks.iter().position(|t| t.id == id);
    
    match task_idx {
        Some(idx) => {
            let task_name = tasks[idx].name.clone();
            
            if Confirm::new()
                .with_prompt(format!("Remove task \"{}\"?", task_name))
                .default(false)
                .interact()
                .unwrap_or(false)
            {
                tasks.remove(idx);
                for (i, task) in tasks.iter_mut().enumerate() {
                    task.id = i + 1;
                }
                
                match save_tasks(&tasks) {
                    Ok(_) => {
                        println!(
                            "{} {}",
                            "✓ Removed task:".green().bold(),
                            task_name.bright_white()
                        );
                    }
                    Err(e) => {
                        println!("{} {}", "Error removing task:".red().bold(), e);
                    }
                }
            }
        }
        None => {
            println!("{}", format!("Task with ID {} not found", id).red());
        }
    }
}

pub fn setup_config() {
    println!("{}", "Let's set up RTask configuration".bright_green().bold());
    
    let mut config = load_config();
    
    let tasks_dir: String = Input::new()
        .with_prompt("Where should tasks be stored?")
        .with_initial_text(&config.tasks_dir)
        .interact_text()
        .unwrap();
    
    let user_name: String = Input::new()
        .with_prompt("What's your name?")
        .with_initial_text(&config.user_name)
        .interact_text()
        .unwrap();
    
    let priorities = vec!["low", "medium", "high"];
    let default_priority_idx = priorities
        .iter()
        .position(|&p| p == config.default_priority)
        .unwrap_or(1);
    
    let default_priority_idx = Select::new()
        .with_prompt("Choose default priority:")
        .items(&priorities)
        .default(default_priority_idx)
        .interact()
        .unwrap();
    
    config.tasks_dir = tasks_dir;
    config.user_name = user_name;
    config.default_priority = priorities[default_priority_idx].to_string();
    
    match save_config(&config) {
        Ok(_) => {
            println!("{}", "✓ Configuration saved successfully".green());
            let tasks_dir = Path::new(&config.tasks_dir);
            if !tasks_dir.exists() {
                if let Err(e) = fs::create_dir_all(tasks_dir) {
                    println!(
                        "{} {}",
                        "Error creating tasks directory:".red().bold(),
                        e.to_string()
                    );
                } else {
                    println!("{}", "✓ Created tasks directory".green());
                }
            }
        }
        Err(e) => {
            println!("{} {}", "Error saving configuration:".red().bold(), e);
        }
    }
}

pub fn interactive_mode() {
    loop {
        println!("\n{}", "RTask Interactive Mode".cyan().bold());
        println!("{}", "---------------------".cyan());
        
        let options = vec![
            "Add a task",
            "List tasks",
            "Today's tasks",
            "Complete a task",
            "Update task progress",
            "Manage subtasks",
            "Remove a task",
            "Show task details",
            "Configure",
            "Exit",
        ];
        
        let selection = Select::new()
            .with_prompt("Choose an action")
            .items(&options)
            .default(0)
            .interact();
        
        match selection {
            Ok(0) => interactive_add_task(),
            Ok(1) => interactive_list_tasks(),
            Ok(2) => show_today_tasks(),
            Ok(3) => interactive_complete_task(),
            Ok(4) => interactive_update_progress(),
            Ok(5) => crate::subtasks::interactive_manage_subtasks(),
            Ok(6) => interactive_remove_task(),
            Ok(7) => interactive_show_task(),
            Ok(8) => setup_config(),
            Ok(9) | _ => break,
        }
    }
    
    println!("{}", "bye byee!".bright_blue());
}

pub fn interactive_add_task() {
    let name: String = Input::new()
        .with_prompt("Task name")
        .interact_text()
        .unwrap();
    
    let priorities = vec!["low", "medium", "high"];
    let config = load_config();
    let default_priority_idx = priorities
        .iter()
        .position(|&p| p == config.default_priority)
        .unwrap_or(1);
    
    let priority_idx = Select::new()
        .with_prompt("Priority")
        .items(&priorities)
        .default(default_priority_idx)
        .interact()
        .unwrap();
    
    let priority = priorities[priority_idx].to_string();
    
    let has_due_date = Confirm::new()
        .with_prompt("Set a due date?")
        .default(false)
        .interact()
        .unwrap();
    
    let due_date = if has_due_date {
        let due: String = Input::new()
            .with_prompt("Due date (YYYY-MM-DD)")
            .interact_text()
            .unwrap();
        Some(due)
    } else {
        None
    };
    
    let has_tags = Confirm::new()
        .with_prompt("Add tags?")
        .default(false)
        .interact()
        .unwrap();
    
    let tags = if has_tags {
        let tags_input: String = Input::new()
            .with_prompt("Tags (comma separated)")
            .interact_text()
            .unwrap();
        Some(tags_input)
    } else {
        None
    };
    
    add_task(name, priority, due_date, tags);
    
    let add_subtasks = Confirm::new()
        .with_prompt("Add subtasks now?")
        .default(false)
        .interact()
        .unwrap();
    
    if add_subtasks {
        let tasks = load_tasks();
        if let Some(task) = tasks.last() {
            crate::subtasks::interactive_add_subtasks(task.id);
        }
    }
}

pub fn interactive_list_tasks() {
    let options = vec!["All tasks", "Pending tasks", "Completed tasks"];
    let selection = Select::new()
        .with_prompt("What tasks to show")
        .items(&options)
        .default(1)
        .interact()
        .unwrap();
    
    let has_filter = Confirm::new()
        .with_prompt("Filter by tag?")
        .default(false)
        .interact()
        .unwrap();
    
    let filter = if has_filter {
        let filter_input: String = Input::new()
            .with_prompt("Tag filter")
            .interact_text()
            .unwrap();
        Some(filter_input)
    } else {
        None
    };
    
    match selection {
        0 => list_tasks(filter, true, false),
        1 => list_tasks(filter, false, false),
        2 => list_tasks(filter, false, true),
        _ => {}
    }
}

pub fn interactive_complete_task() {
    let tasks = load_tasks();
    let pending_tasks: Vec<&Task> = tasks.iter().filter(|t| t.status == "pending").collect();
    
    if pending_tasks.is_empty() {
        println!("{}", "No pending tasks to complete".yellow());
        return;
    }
    
    let task_names: Vec<String> = pending_tasks
        .iter()
        .map(|t| format!("[{}] {}", t.id, t.name))
        .collect();
    
    let selection = Select::new()
        .with_prompt("Select task to complete")
        .items(&task_names)
        .interact();
    
    match selection {
        Ok(idx) => {
            let task_id = pending_tasks[idx].id;
            complete_task(task_id);
        }
        Err(_) => {}
    }
}

pub fn interactive_remove_task() {
    let tasks = load_tasks();
    
    if tasks.is_empty() {
        println!("{}", "No tasks to remove".yellow());
        return;
    }
    
    let task_names: Vec<String> = tasks
        .iter()
        .map(|t| format!("[{}] {} ({})", t.id, t.name, t.status))
        .collect();
    
    let selection = Select::new()
        .with_prompt("Select task to remove")
        .items(&task_names)
        .interact();
    
    match selection {
        Ok(idx) => {
            let task_id = tasks[idx].id;
            remove_task(task_id);
        }
        Err(_) => {}
    }
}

pub fn show_task_details(id: usize) {
    let tasks = load_tasks();
    let task = tasks.iter().find(|t| t.id == id);
    
    match task {
        Some(task) => {
            println!("{}", "Task Details".cyan().bold());
            println!("{}", "------------".cyan());
            println!("{}: {}", "ID".yellow(), task.id);
            println!("{}: {}", "Name".yellow(), task.name);
            println!("{}: {}", "Priority".yellow(), format_priority(&task.priority));
            println!("{}: {}", "Status".yellow(), format_status(&task.status));
            println!("{}: {}", "Progress".yellow(), format_progress_bar(task.progress));
            
            if let Some(due) = &task.due_date {
                println!("{}: {}", "Due Date".yellow(), due);
            }
            
            if !task.tags.is_empty() {
                println!("{}: {}", "Tags".yellow(), task.tags.join(", "));
            }
            
            println!("{}: {}", "Created".yellow(), task.created_at);
            
            if let Some(completed) = &task.completed_at {
                println!("{}: {}", "Completed".yellow(), completed);
            }
            
            if !task.subtasks.is_empty() {
                println!("\n{}", "Subtasks:".cyan().bold());
                for (i, subtask) in task.subtasks.iter().enumerate() {
                    let status = if subtask.completed {
                        "[✓]".green()
                    } else {
                        "[ ]".yellow()
                    };
                    println!("{} {}: {}", status, (i + 1).to_string().cyan(), subtask.name);
                }
            }
        },
        None => {
            println!("{}", format!("Task with ID {} not found", id).red());
        }
    }
}

pub fn interactive_show_task() {
    let tasks = load_tasks();
    
    if tasks.is_empty() {
        println!("{}", "No tasks found".yellow());
        return;
    }
    
    let task_names: Vec<String> = tasks
        .iter()
        .map(|t| format!("[{}] {} ({})", t.id, t.name, t.status))
        .collect();
    
    let selection = Select::new()
        .with_prompt("Select task to view")
        .items(&task_names)
        .interact();
    
    match selection {
        Ok(idx) => {
            let task_id = tasks[idx].id;
            show_task_details(task_id);
        }
        Err(_) => {}
    }
}

pub fn show_today_tasks() {
    let tasks = load_tasks();
    let today = Local::now().format("%Y-%m-%d").to_string();
    
    let today_tasks: Vec<Task> = tasks
        .into_iter()
        .filter(|task| {
            task.status != "done" && 
            task.due_date.as_ref().map_or(false, |due| due == &today)
        })
        .collect();
    
    if today_tasks.is_empty() {
        println!("{}", "No tasks due today!".green());
        return;
    }
    
    println!("{} {}", "Tasks due today:".cyan().bold(), today.bright_white());
    let colored_tasks: Vec<Task> = today_tasks
        .iter()
        .map(|task| {
            let mut colored_task = task.clone();
            colored_task.priority = format_priority(&task.priority);
            colored_task.status = format_status(&task.status);
            colored_task
        })
        .collect();
    println!();
    for task in &colored_tasks {
        let id_str = format!("[{}]", task.id).cyan().bold();
        let priority_str = format!("[{}]", task.priority);
        let status_str = format!("[{}]", task.status);
        let progress_str = format_progress_bar(task.progress);
        
        let due_str = if let Some(due) = &task.due_date {
            format!("(Due: {})", due).yellow()
        } else {
            "".normal()
        };
        
        let tags_str = if !task.tags.is_empty() {
            format!("#{}",  task.tags.join(" #")).bright_blue()
        } else {
            "".normal()
        };
        
        let subtasks_str = if !task.subtasks.is_empty() {
            let completed = task.subtasks.iter().filter(|s| s.completed).count();
            format!("[{}/{}]", completed, task.subtasks.len()).bright_magenta()
        } else {
            "".normal()
        };
        
        println!("{} {} {} {} {} {} {} {}", 
            id_str,
            priority_str,
            status_str,
            progress_str,
            task.name.bright_white(),
            due_str,
            tags_str,
            subtasks_str
        );
    }
    println!();
}

pub fn update_task_progress(id: usize, progress: u8) {
    let mut tasks = load_tasks();
    
    let task_idx = tasks.iter().position(|t| t.id == id);
    
    match task_idx {
        Some(idx) => {
            if tasks[idx].status == "done" {
                println!("{}", "Cannot update progress of completed task".yellow());
                return;
            }
            
            let progress = progress.min(100);
            tasks[idx].progress = progress;
            
            if progress == 100 {
                tasks[idx].status = "done".to_string();
                tasks[idx].completed_at = Some(Local::now().format("%Y-%m-%d %H:%M").to_string());
                
                for subtask in &mut tasks[idx].subtasks {
                    subtask.completed = true;
                }
            }
            
            match save_tasks(&tasks) {
                Ok(_) => {
                    println!(
                        "{} {} {}",
                        "✓ Updated progress for task:".green().bold(),
                        tasks[idx].name.bright_white(),
                        format!("({}%)", progress).cyan()
                    );
                }
                Err(e) => {
                    println!("{} {}", "Error updating task progress:".red().bold(), e);
                }
            }
        }
        None => {
            println!("{}", format!("Task with ID {} not found", id).red());
        }
    }
}

pub fn interactive_update_progress() {
    let tasks = load_tasks();
    let pending_tasks: Vec<&Task> = tasks.iter().filter(|t| t.status == "pending").collect();
    
    if pending_tasks.is_empty() {
        println!("{}", "No pending tasks to update".yellow());
        return;
    }
    
    let task_names: Vec<String> = pending_tasks
        .iter()
        .map(|t| format!("[{}] {} ({}%)", t.id, t.name, t.progress))
        .collect();
    
    let selection = Select::new()
        .with_prompt("Select task to update progress")
        .items(&task_names)
        .interact();
    
    match selection {
        Ok(idx) => {
            let task_id = pending_tasks[idx].id;
            let current_progress = pending_tasks[idx].progress;
            
            let progress: u8 = Input::new()
                .with_prompt("Enter progress percentage (0-100)")
                .with_initial_text(&current_progress.to_string())
                .interact_text()
                .unwrap_or(current_progress);
            
            update_task_progress(task_id, progress);
        }
        Err(_) => {}
    }
}

pub fn print_welcome_banner() {
    println!("{}", r#"
 _____  _____         _     
|  __ \|_   _|       | |    
| |__) | | | __ _ ___| | __ 
|  _  /  | |/ _` / __| |/ / 
| | \ \ _| | (_| \__ \   <  
|_|  \_\_____\__,_|___/_|\_\ 
                            
"#.bright_cyan());
    println!("{}", "a task tracker in rust because why not?".bright_blue());
    println!("{}", "----------------------------------------".bright_blue());
    println!("Created by Miisan");
    println!();
}