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
    },
    Done {
        name: String,
        date: chrono::NaiveDate
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
            Commands::Done{ name, date } => {
                let habit = habits.return_mut_by_name(&name);
                if let Some(h) = habit {
                    h.add_completed_day(date)
                } else {
                    eprintln!("Could not find find habit with name `{name}`");
                }
            }
            _ => todo!()
        }

        std::fs::write(path, habits.to_json()).expect("Could not write to {path}");
    }
}
