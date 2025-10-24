pub mod plugin;
pub mod run;
pub mod script;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A powerful script runner
#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run a script with optional arguments
    #[command(visible_alias = "r")]
    Run {
        /// Name of the script to run
        name: String,
        /// Specify script extension
        #[arg(short = 'x', long)]
        extension: Option<String>,
        /// Specify plugin to use (overrides auto-detection)
        #[arg(short = 'p', long)]
        plugin: Option<String>,

        /// Arguments to pass to the script (use -- to separate from rune args)
        #[arg(last = true)]
        args: Vec<String>,
    },

    /// Manage scripts (add, remove, list, create new)
    #[command(visible_alias = "s")]
    Script {
        #[command(subcommand)]
        command: ScriptCommands,
    },

    /// Manage plugins (add, remove, list)
    #[command(visible_alias = "p")]
    Plugin {
        #[command(subcommand)]
        command: PluginCommands,
    },
}
#[derive(Subcommand, Debug)]
pub enum ScriptCommands {
    /// Add a script from file path
    #[command(visible_alias = "a")]
    Add {
        /// Path to the script file
        path: PathBuf,

        /// Force overwrite if script already exists
        #[arg(short, long)]
        force: bool,
    },

    /// Remove a script by name
    #[command(visible_aliases = ["rm", "delete"])]
    Remove {
        /// Name of the script to remove
        name: String,

        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,

        /// Specify script extension
        #[arg(short = 'x', long)]
        extension: Option<String>,
    },

    /// List all available scripts
    #[command(visible_aliases = ["ls", "l"])]
    List {
        /// Display in plain text format (one per line)
        #[arg(short = '1', long)]
        plain: bool,
    },

    /// Create a new script from template
    #[command(visible_alias = "n")]
    New {
        /// Name for the new script
        name: String,
    },
    #[command(visible_alias = "e")]
    Edit {
        /// Name for the script
        name: String,
        /// Specify script extension
        #[arg(short = 'x', long)]
        extension: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum PluginCommands {
    /// Add a plugin from file path
    #[command(visible_alias = "a")]
    Add {
        /// Path to the plugin file
        path: PathBuf,

        /// Force overwrite if plugin already exists
        #[arg(short, long)]
        force: bool,
    },

    /// Remove a plugin by name
    #[command(visible_aliases = ["rm", "delete"])]
    Remove {
        /// Name of the plugin to remove
        name: String,

        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },

    /// List all installed plugins
    #[command(visible_aliases = ["ls", "l"])]
    List {
        /// Display in plain text format (one per line)
        #[arg(short = '1', long)]
        plain: bool,
    },
    /// Show plugin info
    #[command(visible_aliases = ["i"])]
    Info {
        /// Name of the plugin
        name: String,
    },
    /// Export plugin
    #[command(visible_aliases = ["e"])]
    Export {
        /// Name of the plugin
        name: String,
        #[arg(short = 'o', long)]
        output: Option<PathBuf>,
    },
}
