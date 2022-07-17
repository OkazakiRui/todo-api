use anyhow::{Ok, Result};

fn run() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
