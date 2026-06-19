use std::path::PathBuf;

pub struct Shell {
    pub current_directory: PathBuf,
    pub git_branch_cache: Option<String>,
    pub visited_directories: Vec<PathBuf>,
}