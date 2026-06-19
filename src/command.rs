#![allow(non_camel_case_types)]
pub enum Command {
    pwd,
    cd(String),
    touch(String),
    ls(Option<String>),
    rm {
        recursive: bool,
        path: String,
    },
    exit,
    external {
        program: String,
        args: Vec<String>,
    },
}