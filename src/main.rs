mod command;
mod executor;
mod parser;
mod path;
mod prompt;
mod shell;

use rustyline::DefaultEditor;
use std::process::Command as ProcessCommand;

use crate::command::Command;
use crate::parser::parse_command;
use crate::path::resolve_path;
use crate::prompt::git_branch;
use crate::prompt::prompt_path;
use crate::shell::Shell;

fn main() {
    let mut shell = Shell {
        current_directory: std::env::current_dir().unwrap(),
        git_branch_cache: None,
        visited_directories: Vec::new(),
    };

    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let path = prompt_path(&shell);

        let prompt = if let Some(branch) = git_branch(&shell) {
            format!("{} [{}]> ", path, branch)
        } else {
            format!("{}> ", path)
        };

        let input = match rl.readline(&prompt) {
            Ok(line) => line,
            Err(_) => break,
        };

        rl.add_history_entry(input.as_str()).ok();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        let command = parse_command(&parts);

        if !executor::execute(&mut shell, command) {
            break;
        }
    }
}
