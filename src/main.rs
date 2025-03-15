mod model;
mod commands;
mod subtasks;

use clap::{Parser, Subcommand};
use model::is_config_exists;
use commands::{
    add_task, complete_task, interactive_mode, list_tasks, 
    print_welcome_banner, remove_task, setup_config,
    show_task_details, show_today_tasks, update_task_progress
};
use subtasks::{
    add_subtask, toggle_subtask, remove_subtask
};

fn main() {
    if !is_config_exists() {
        print_welcome_banner();
        setup_config();
    }

    let args = RTaskArgs::parse();
    match args.command {
        Some(Commands::Add { name, priority, due, tags }) => {
            add_task(name, priority, due, tags);
        }
        Some(Commands::List { filter, all, completed }) => {
            list_tasks(filter, all, completed);
        }
        Some(Commands::Complete { id }) => {
            complete_task(id);
        }
        Some(Commands::Remove { id }) => {
            remove_task(id);
        }
        Some(Commands::Show { id }) => {
            show_task_details(id);
        }
        Some(Commands::Config {}) => {
            setup_config();
        }
        Some(Commands::Today {}) => {
            show_today_tasks();
        }
        Some(Commands::Progress { id, percentage }) => {
            update_task_progress(id, percentage);
        }
        Some(Commands::Subtask { command }) => {
            match command {
                SubtaskCommands::Add { task_id, name } => {
                    add_subtask(task_id, name);
                }
                SubtaskCommands::Toggle { task_id, subtask_index } => {
                    toggle_subtask(task_id, subtask_index - 1); // Convert to 0-based index
                }
                SubtaskCommands::Remove { task_id, subtask_index } => {
                    remove_subtask(task_id, subtask_index - 1); // Convert to 0-based index
                }
            }
        }
        None => {
            interactive_mode();
        }
    }
}

#[derive(Parser)]
#[command(
    name = "rtask",
    author = "Miisan",
    version = "0.1.0",
    about = "A simple CLI task tracker",
    long_about = "RTask - A beautiful CLI task tracker written in Rust"
)]
struct RTaskArgs {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(visible_alias = "a")]
    Add {
        name: String,
        #[arg(short, long, default_value = "medium")]
        priority: String,
        #[arg(short, long)]
        due: Option<String>,
        
        #[arg(short, long)]
        tags: Option<String>,
    },
    
    #[command(visible_alias = "ls")]
    #[command(visible_alias = "log")]
    List {
        #[arg(short, long)]
        filter: Option<String>,
        
        #[arg(short, long)]
        all: bool,
        
        #[arg(short, long)]
        completed: bool,
    },
    
    #[command(visible_alias = "done")]
    #[command(visible_alias = "c")]
    Complete {
        id: usize,
    },
    
    #[command(visible_alias = "rm")]
    #[command(visible_alias = "delete")]
    Remove {
        id: usize,
    },
    
    #[command(visible_alias = "s")]
    Show {
        id: usize,
    },
    
    #[command(visible_alias = "cfg")]
    Config {},
    
    #[command(visible_alias = "td")]
    Today {},

    #[command(visible_alias = "prog")]
    #[command(visible_alias = "p")]
    Progress {
        id: usize,
        percentage: u8,
    },

    #[command(visible_alias = "sub")]
    Subtask {
        #[command(subcommand)]
        command: SubtaskCommands,
    },
}

#[derive(Subcommand)]
enum SubtaskCommands {
    #[command(visible_alias = "a")]
    Add {
        #[arg(short, long)]
        task_id: usize,
        name: String,
    },
    
    #[command(visible_alias = "t")]
    #[command(visible_alias = "check")]
    Toggle {
        #[arg(short, long)]
        task_id: usize,
        
        #[arg(short, long)]
        subtask_index: usize,
    },
    
    #[command(visible_alias = "rm")]
    Remove {
        #[arg(short, long)]
        task_id: usize,
        
        #[arg(short, long)]
        subtask_index: usize,
    },
}