use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        print!("%");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();

        let mut child = Command::new(command).args(parts).spawn().unwrap();

        child.wait();
    }
}
