use std::env;
use std::path::PathBuf;

use crate::shell::Shell;

pub fn resolve_path(shell: &Shell, input: &str) -> PathBuf {
    let input = input.replace("\\", "/");

    if input == "~" {
        return PathBuf::from(env::var("USERPROFILE").unwrap());
    }

    if input.starts_with("~/") {
        let home = env::var("USERPROFILE").unwrap();

        let relative = input.trim_start_matches("~/");

        return PathBuf::from(home).join(relative);
    }

    if input == ".." {
        return shell
            .current_directory
            .parent()
            .unwrap_or(&shell.current_directory)
            .to_path_buf();
    }

    shell.current_directory.join(input)
}
