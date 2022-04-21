mod habit;

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use crate::habit::{Habits, Habit};


#[derive(Parser)]
struct Cli {

    #[clap(short, long, parse(from_os_str))]
    file: Option<PathBuf>,

    #[clap(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    New {
        #[clap(short, long)]
        name: String
    }
}

fn main() {
    let cli = Cli::parse();

    if let Some(path) = cli.file {
        let mut habits = if path.is_file() {
            let content = std::fs::read_to_string(&path).expect("Could not read from {path}");
            Habits::from_json(&content)
        } else {
            Habits::new()
        };

        match cli.command {
            Commands::New{ name } =>  {
                habits.add(Habit::start_today(name))
            },
            _ => todo!()
        }

        std::fs::write(path, habits.to_json()).expect("Could not write to {path}");
    }
}
