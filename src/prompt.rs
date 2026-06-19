use crate::shell::Shell;
use std::process::Command as ProcessCommand;

pub fn prompt_path(shell: &Shell) -> String {
    let home = std::env::var("USERPROFILE").unwrap();

    let current = shell.current_directory.to_string_lossy();

    if current.starts_with(&home) {
        current.replacen(&home, "~", 1).replace("\\", "/")
    } else {
        current.to_string()
    }
}

pub fn git_branch(shell: &Shell) -> Option<String> {
    let output = ProcessCommand::new("git")
        .args(["branch", "--show-current"])
        .current_dir(&shell.current_directory)
        .output()
        .ok()?;

    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if branch.is_empty() {
        None
    } else {
        Some(branch)
    }
}
