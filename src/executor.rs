use std::process::Command as ProcessCommand;

use crate::command::Command;
use crate::path::resolve_path;
use crate::prompt::prompt_path;
use crate::shell::Shell;

pub fn execute(shell: &mut Shell, command: Command) -> bool {
    match command {
        Command::pwd => {
            println!("{}", prompt_path(&shell));
        }

        Command::cd(path) => {
            if path.is_empty() {
                println!("cd requires a path");
                return true;
            }

            let destination = resolve_path(shell, &path);

            if destination.exists() {
                shell.current_directory = destination;
            } else {
                println!("Directory does not exist");
            }
        }

        Command::ls(_) => {
            let entries = std::fs::read_dir(&shell.current_directory);

            match entries {
                Ok(entries) => {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            println!("{}", entry.file_name().to_string_lossy());
                        }
                    }
                }

                Err(err) => {
                    println!("{}", err);
                }
            }
        }

        Command::touch(path) => match std::fs::File::create(path) {
            Ok(_) => {}

            Err(err) => {
                println!("{}", err);
            }
        },

        Command::rm { recursive, path } => {
            let target = resolve_path(shell, &path);

            if target.is_dir() {
                if recursive {
                    if let Err(err) = std::fs::remove_dir_all(&target) {
                        println!("{}", err);
                    }
                } else {
                    println!("Directory removal requires -rf");
                }
            } else if target.is_file() {
                if let Err(err) = std::fs::remove_file(&target) {
                    println!("{}", err);
                }
            } else {
                println!("Path not found!");
            }
        }

        Command::exit => {
            return false;
        }

        Command::external { program, args } => {
            match ProcessCommand::new(&program)
                .args(&args)
                .current_dir(&shell.current_directory)
                .status()
            {
                Ok(status) => {
                    println!("Exited with: {}", status);
                }

                Err(err) => {
                    println!("Failed: {}", err);
                }
            }
        }

        _ => {}
    }

    return true;
}
