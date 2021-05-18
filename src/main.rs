use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        print!("% ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();

        match command {
            "cd" => {
                let new_dir = parts.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            "exit" => return,
            command => {
                let child = Command::new(command).args(parts).spawn();
                match child {
                    Ok(mut child) => {
                        child.wait();
                    }
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
    }
}
