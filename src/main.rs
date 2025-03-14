mod model;
mod commands;

use clap::{Parser, Subcommand};
use model::is_config_exists;
use commands::{
    add_task, complete_task, interactive_mode, list_tasks, 
    print_welcome_banner, remove_task, setup_config,
    show_task_details, show_today_tasks
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
    
    #[command(visible_alias = "show")]
    #[command(visible_alias = "s")]
    Show {
        id: usize,
    },
    
    #[command(visible_alias = "cfg")]
    Config {},
    
    #[command(visible_alias = "td")]
    Today {},
}