mod commands;
mod core;
mod utils;

use clap::Parser;
use commands::{Cli, Commands};

use crate::commands::{plugin, run, script};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run {
            name,
            extension,
            plugin,
            args,
        } => {
            if let Err(e) = run::run(name, extension, plugin, args) {
                eprintln!("Error: {:#}", e)
            }
        }
        Commands::Plugin { command } => match command {
            commands::PluginCommands::Add { path, force } => {
                if let Err(e) = plugin::add(path, force) {
                    eprintln!("Error: {:#}", e)
                }
            }
            commands::PluginCommands::Remove { name, yes } => {
                if let Err(e) = plugin::remove(name, yes) {
                    eprintln!("Error: {:#}", e)
                }
            }
            commands::PluginCommands::List { plain } => {
                if let Err(e) = plugin::list(plain) {
                    eprintln!("Error: {:#}", e)
                }
            }
            commands::PluginCommands::Info { name } => {
                if let Err(e) = plugin::info(name) {
                    eprintln!("Error: {:#}", e)
                }
            }
            commands::PluginCommands::Export { name, output, format } => {
                if let Err(e) = plugin::export(name, output, format) {
                    eprintln!("Error: {:#}", e)
                }
            }
        },
        Commands::Script { command } => match command {
            commands::ScriptCommands::Add { path, force } => {
                if let Err(e) = script::add(path, force) {
                    eprintln!("Error: {:#}", e)
                }
            }
            commands::ScriptCommands::Remove {
                name,
                yes,
                extension,
            } => {
                if let Err(e) = script::remove(name, yes, extension) {
                    eprintln!("Error: {:#}", e)
                }
            }
            commands::ScriptCommands::List { plain } => {
                if let Err(e) = script::list(plain) {
                    eprintln!("Error: {:#}", e)
                }
            }
            commands::ScriptCommands::New { name } => {
                if let Err(e) = script::new(name) {
                    eprintln!("Error: {:#}", e)
                }
            }
            commands::ScriptCommands::Edit { name, extension } => {
                if let Err(e) = script::edit(name, extension) {
                    eprintln!("Error: {:#}", e)
                }
            }
        },
    }
}
