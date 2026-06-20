use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    println!("Welcome to ksh v0.0.1");

    loop {
        let cur_dir = env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("/"));
        let dir_string = cur_dir.to_string_lossy();

        print!("kid@kidux:{}$ ", dir_string);
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
            println!("\nCommands:\n\t\"help\", \"clear\", \"echo\", \"whoami\", \"pwd\", \"cd\", \"ls\"");
            continue;
        }

        if cmd == "cd" {
            let target_str = args.first().unwrap_or(&"/");
            let new_dir = Path::new(target_str);
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