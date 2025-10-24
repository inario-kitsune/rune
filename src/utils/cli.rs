use anyhow::{Context, Result};
use std::io::{self, Write};

pub fn prompt_confirm(prompt: &str, default_yes: bool) -> Result<bool> {
    let hint = if default_yes { "[Y/n" } else { "[y/N]" };
    println!("{} {}", prompt, hint);
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .context("Failed to read user input")?;

        match input.trim().to_ascii_lowercase().as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            "" => return Ok(default_yes), // 空输入使用默认值
            _ => println!("Invalid input. Please enter 'y' or 'n'."),
        }
    }
}
