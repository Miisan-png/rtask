use colored::*;
use dialoguer::{Confirm, Input, Select};
use chrono::Local;

use crate::model::SubTask;
use crate::commands::{load_tasks, save_tasks};

pub fn add_subtask(id: usize, name: String) {
    let mut tasks = load_tasks();
    
    let task_idx = tasks.iter().position(|t| t.id == id);
    
    match task_idx {
        Some(idx) => {
            let subtask = SubTask {
                name,
                completed: false,
            };
            
            tasks[idx].subtasks.push(subtask);
            
            match save_tasks(&tasks) {
                Ok(_) => {
                    println!(
                        "{} {} {}",
                        "✓ Added subtask to:".green().bold(),
                        tasks[idx].name.bright_white(),
                        format!("(#{}/{})", tasks[idx].subtasks.len(), tasks[idx].subtasks.len()).cyan()
                    );
                }
                Err(e) => {
                    println!("{} {}", "Error adding subtask:".red().bold(), e);
                }
            }
        }
        None => {
            println!("{}", format!("Task with ID {} not found", id).red());
        }
    }
}

pub fn toggle_subtask(id: usize, subtask_idx: usize) {
    let mut tasks = load_tasks();
    
    let task_idx = tasks.iter().position(|t| t.id == id);
    
    match task_idx {
        Some(idx) => {
            if subtask_idx >= tasks[idx].subtasks.len() {
                println!("{}", format!("Subtask {} not found", subtask_idx + 1).red());
                return;
            }
            
            tasks[idx].subtasks[subtask_idx].completed = !tasks[idx].subtasks[subtask_idx].completed;
            
            // Update task progress based on subtasks
            if !tasks[idx].subtasks.is_empty() {
                let completed_count = tasks[idx].subtasks.iter().filter(|s| s.completed).count();
                let progress = (completed_count as f32 / tasks[idx].subtasks.len() as f32 * 100.0) as u8;
                tasks[idx].progress = progress;
                
                if progress == 100 {
                    tasks[idx].status = "done".to_string();
                    tasks[idx].completed_at = Some(Local::now().format("%Y-%m-%d %H:%M").to_string());
                }
            }
            
            match save_tasks(&tasks) {
                Ok(_) => {
                    let status = if tasks[idx].subtasks[subtask_idx].completed {
                        "completed".green()
                    } else {
                        "uncompleted".yellow()
                    };
                    
                    println!(
                        "{} {} {}",
                        "✓ Subtask:".green().bold(),
                        tasks[idx].subtasks[subtask_idx].name.bright_white(),
                        format!("({}) - {}", status, tasks[idx].name).cyan()
                    );
                }
                Err(e) => {
                    println!("{} {}", "Error updating subtask:".red().bold(), e);
                }
            }
        }
        None => {
            println!("{}", format!("Task with ID {} not found", id).red());
        }
    }
}

pub fn remove_subtask(id: usize, subtask_idx: usize) {
    let mut tasks = load_tasks();
    
    let task_idx = tasks.iter().position(|t| t.id == id);
    
    match task_idx {
        Some(idx) => {
            if subtask_idx >= tasks[idx].subtasks.len() {
                println!("{}", format!("Subtask {} not found", subtask_idx + 1).red());
                return;
            }
            
            let subtask_name = tasks[idx].subtasks[subtask_idx].name.clone();
            
            if Confirm::new()
                .with_prompt(format!("Remove subtask \"{}\"?", subtask_name))
                .default(false)
                .interact()
                .unwrap_or(false)
            {
                tasks[idx].subtasks.remove(subtask_idx);
                
                // Update task progress based on remaining subtasks
                if !tasks[idx].subtasks.is_empty() {
                    let completed_count = tasks[idx].subtasks.iter().filter(|s| s.completed).count();
                    let progress = (completed_count as f32 / tasks[idx].subtasks.len() as f32 * 100.0) as u8;
                    tasks[idx].progress = progress;
                }
                
                match save_tasks(&tasks) {
                    Ok(_) => {
                        println!(
                            "{} {}",
                            "✓ Removed subtask:".green().bold(),
                            subtask_name.bright_white()
                        );
                    }
                    Err(e) => {
                        println!("{} {}", "Error removing subtask:".red().bold(), e);
                    }
                }
            }
        }
        None => {
            println!("{}", format!("Task with ID {} not found", id).red());
        }
    }
}

pub fn interactive_manage_subtasks() {
    let tasks = load_tasks();
    
    if tasks.is_empty() {
        println!("{}", "No tasks found".yellow());
        return;
    }
    
    let task_names: Vec<String> = tasks
        .iter()
        .map(|t| {
            let subtask_count = t.subtasks.len();
            let completed = t.subtasks.iter().filter(|s| s.completed).count();
            
            if subtask_count > 0 {
                format!("[{}] {} ({}/{} subtasks)", t.id, t.name, completed, subtask_count)
            } else {
                format!("[{}] {}", t.id, t.name)
            }
        })
        .collect();
    
    let selection = Select::new()
        .with_prompt("Select task to manage subtasks")
        .items(&task_names)
        .interact();
    
    match selection {
        Ok(idx) => {
            let task_id = tasks[idx].id;
            interactive_subtask_menu(task_id);
        }
        Err(_) => {}
    }
}

pub fn interactive_subtask_menu(id: usize) {
    loop {
        let tasks = load_tasks();
        let task = match tasks.iter().find(|t| t.id == id) {
            Some(t) => t,
            None => {
                println!("{}", format!("Task with ID {} not found", id).red());
                return;
            }
        };

        println!("\n{} {}", "Subtasks for:".cyan().bold(), task.name.bright_white());
        
        let options = vec![
            "Add new subtask",
            "Toggle subtask completion",
            "Remove subtask",
            "Back to main menu",
        ];
        
        let selection = Select::new()
            .with_prompt("Choose an action")
            .items(&options)
            .default(0)
            .interact();
        
        match selection {
            Ok(0) => interactive_add_subtasks(id),
            Ok(1) => interactive_toggle_subtask(id),
            Ok(2) => interactive_remove_subtask(id),
            Ok(3) | _ => break,
        }
    }
}

pub fn interactive_add_subtasks(id: usize) {
    let mut continue_adding = true;
    
    while continue_adding {
        let name: String = Input::new()
            .with_prompt("Subtask name")
            .interact_text()
            .unwrap();
        
        add_subtask(id, name);
        
        continue_adding = Confirm::new()
            .with_prompt("Add another subtask?")
            .default(false)
            .interact()
            .unwrap_or(false);
    }
}

pub fn interactive_toggle_subtask(id: usize) {
    let tasks = load_tasks();
    let task = tasks.iter().find(|t| t.id == id);
    
    match task {
        Some(task) => {
            if task.subtasks.is_empty() {
                println!("{}", "This task has no subtasks".yellow());
                return;
            }
            
            let subtask_names: Vec<String> = task.subtasks
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    let status = if s.completed {
                        "[✓]".green()
                    } else {
                        "[ ]".yellow()
                    };
                    format!("{} {}: {}", status, i + 1, s.name)
                })
                .collect();
            
            let selection = Select::new()
                .with_prompt("Select subtask to toggle")
                .items(&subtask_names)
                .interact();
            
            match selection {
                Ok(idx) => {
                    toggle_subtask(id, idx);
                }
                Err(_) => {}
            }
        }
        None => {
            println!("{}", format!("Task with ID {} not found", id).red());
        }
    }
}

pub fn interactive_remove_subtask(id: usize) {
    let tasks = load_tasks();
    let task = tasks.iter().find(|t| t.id == id);
    
    match task {
        Some(task) => {
            if task.subtasks.is_empty() {
                println!("{}", "This task has no subtasks".yellow());
                return;
            }
            
            let subtask_names: Vec<String> = task.subtasks
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    let status = if s.completed {
                        "[✓]".green()
                    } else {
                        "[ ]".yellow()
                    };
                    format!("{} {}: {}", status, i + 1, s.name)
                })
                .collect();
            
            let selection = Select::new()
                .with_prompt("Select subtask to remove")
                .items(&subtask_names)
                .interact();
            
            match selection {
                Ok(idx) => {
                    remove_subtask(id, idx);
                }
                Err(_) => {}
            }
        }
        None => {
            println!("{}", format!("Task with ID {} not found", id).red());
        }
    }
}