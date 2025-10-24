use anyhow::{Result, anyhow};
use std::{env, path::PathBuf};

pub fn get_script_path() -> Result<PathBuf> {
    match env::var("RUNE_REPO") {
        Ok(v) => Ok(PathBuf::from(v)),
        Err(_) => Ok(get_data_home()?.join("scripts")),
    }
}
pub fn get_plugin_path() -> Result<PathBuf> {
    match env::var("RUNE_PLUGIN") {
        Ok(v) => Ok(PathBuf::from(v)),
        Err(_) => Ok(get_data_home()?.join("plugin")),
    }
}
pub fn get_plugin_db() -> Result<PathBuf> {
    Ok(get_plugin_path()?.join("plugin.dat"))
}
fn get_data_home() -> Result<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        env::var("APPDATA")
            .ok()
            .map(|p| PathBuf::from(p).join("rune"))
            .or_else(|| {
                env::var("USERPROFILE").ok().map(|p| {
                    PathBuf::from(p)
                        .join("AppData")
                        .join("Roaming")
                        .join("rune")
                })
            })
            .ok_or_else(|| {
                anyhow!("Could not determine data directory: APPDATA or USERPROFILE not set")
            })
    }
    #[cfg(not(target_os = "windows"))]
    {
        env::var("XDG_DATA_HOME")
            .ok()
            .map(|p| PathBuf::from(p).join("rune"))
            .or_else(|| {
                env::var("HOME")
                    .ok()
                    .map(|p| PathBuf::from(p).join(".local").join("share").join("rune"))
            })
            .ok_or_else(|| {
                anyhow!("Could not determine data directory: XDG_DATA_HOME or HOME not set")
            })
    }
}
