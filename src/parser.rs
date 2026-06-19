use crate::command::Command;

pub fn parse_command(parts: &[&str]) -> Command {
    match parts[0] {
        "pwd" => Command::pwd,

        "cd" => {
            let path = parts.get(1).unwrap_or(&"").to_string();

            Command::cd(path)
        }

        "ls" => Command::ls(None),

        "touch" => {
            let path = parts.get(1).unwrap_or(&"").to_string();

            Command::touch(path)
        }

        "rm" => {
            if parts.get(1) == Some(&"-rf") {
                let path = parts.get(2).unwrap_or(&"").to_string();

                Command::rm {
                    recursive: true,
                    path,
                }
            } else {
                let path = parts.get(1).unwrap_or(&"").to_string();

                Command::rm {
                    recursive: false,
                    path,
                }
            }
        }

        "exit" => Command::exit,

        _ => Command::external {
            program: parts[0].to_string(),
            args: parts[1..].iter().map(|s| s.to_string()).collect(),
        },
    }
}
