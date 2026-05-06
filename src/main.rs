#[allow(unused_imports)]
use std::io::{self, Write};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::process::Command;

fn find_executable(command: &str) -> Option<String> {
    let path = std::env::var("PATH").unwrap_or_default();
    for dir in path.split(':') {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if entry.file_name() == command {
                    let mode = entry.metadata().ok()?.permissions().mode();
                    if mode & 0o100 != 0 {
                        return entry.path().to_str().map(|s| s.to_string());
                    }
                }
            }
        }
    }
    None
}

fn main() {

      loop {
         print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().to_string();

        let command: Vec<&str> = input.split_whitespace().collect();
        match command.as_slice() {
            [] => continue,
            ["exit"] => break,
            ["echo", args @ ..] => println!("{}", args.join(" ")),
            ["type", args @ ("exit" | "type" | "echo")] => println!("{} is a shell builtin", args),
            ["type", args @ ..] => {
                let cmd = args.join(" ");
                match find_executable(&cmd) {
                    Some(path) => println!("{} is {}", cmd, path),
                    None => println!("{}: not found", cmd),
                }
            },
            [cmd, args @ ..] => { 
                match find_executable(&cmd) {
                    Some(path) => { Command::new(&path).arg0(cmd).args(args).status().unwrap(); },
                    None => println!("{}: command not found", cmd)
                }
            }
        }
    }
   }
