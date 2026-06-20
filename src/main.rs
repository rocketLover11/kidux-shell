use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let home_var = env::var("HOME").unwrap_or_else(|_| "/".to_string());
    let home_path = PathBuf::from(&home_var);

    println!("Welcome to ksh v0.0.1");

    loop {
        let cur_dir = env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("/"));
        let prompt_dir = match cur_dir.strip_prefix(&home_path) {
            Ok(relative_path) => {
                if relative_path.as_os_str().is_empty() {
                    "~".to_string()
                } else {
                    format!("~/{}", relative_path.display())
                }
            }
            Err(_) => {
                cur_dir.display().to_string()
            }
        };

        print!("kid@kidux:{}$ ", prompt_dir);
        if let Err(e) = io::stdout().flush() {
            eprintln!("Flush error: {}", e);
            continue;
        }

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { continue; }

        let input = input.trim();
        if input.is_empty() { continue; }

        if input == "exit" || input == "quit" {
            break;
        }

        let mut parts = input.split_whitespace();
        let cmd = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        if cmd == "help" {
            println!("\nCommands:\n\t\"help\", \"clear\", \"echo\", \"whoami\", \"pwd\", \"cd\", \"ls\"\n");
            continue;
        }

        if cmd == "cd" {
            let mut target_str = args.first().map(|s| s.to_string()).unwrap_or_else(|| home_var.clone());

            if target_str == "~" {
                target_str = home_var.clone();
            } else if target_str.starts_with("~/") {
                target_str = target_str.replacen("~", &home_var, 1);
            }

            let new_dir = Path::new(&target_str);
            if let Err(e) = env::set_current_dir(&new_dir) {
                eprintln!("cd: error changing directory to {}: {}", target_str, e);
            }
            continue;
        }

        match Command::new(cmd).args(&args).status() {
            Ok(_) => {}
            Err(_) => println!("ksh: Command '{}' could not execute.", cmd),
        }
    }
}