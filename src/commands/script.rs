use crate::{
    core::{executor::CommandExecutor, script::load_scripts},
    utils::{cli::prompt_confirm, fs::get_script_path},
};
use anyhow::{Context, Result, bail};
use std::{env, fs, path::PathBuf};
use tabled::{Table, Tabled, settings::Style};

#[derive(Debug, Tabled)]
struct ScriptListInfo {
    #[tabled(rename = "Index")]
    index: usize,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Extension")]
    extension: String,
}

pub fn list(plain: bool) -> Result<()> {
    let script_path = get_script_path().context("Failed to get script path")?;
    let scripts = load_scripts(script_path)?;
    if scripts.is_empty() {
        println!("No script found");
        return Ok(());
    }
    if plain {
        for script in scripts {
            println!("{}", script.name);
        }
    } else {
        let scripts: Vec<ScriptListInfo> = scripts
            .iter()
            .enumerate()
            .filter_map(|(index, entry)| {
                Some(ScriptListInfo {
                    index,
                    name: entry.name.clone(),
                    extension: entry.extension.clone(),
                })
            })
            .collect();
        let mut table = Table::new(scripts);
        table.with(Style::rounded());
        println!("{}", table);
    };
    Ok(())
}
pub fn add(path: PathBuf, force: bool) -> Result<()> {
    // 检查源文件是否存在
    if !path.exists() {
        bail!("Target script does not exist: {}", path.display());
    }

    // 检查是否是文件
    if !path.is_file() {
        bail!("Target path is not a file: {}", path.display());
    }

    let file_name = path.file_name().context("Failed to get file name")?;

    let script_path = get_script_path().context("Failed to get script path")?;

    let target_path = script_path.join(file_name);

    // 如果目标已存在且没有 force 标志，询问用户
    if target_path.exists() && !force {
        let message = format!(
            "Script '{}' already exists. Overwrite?",
            file_name.to_string_lossy()
        );
        if !prompt_confirm(&message, false)? {
            bail!("Script already exists, operation cancelled");
        }
    }

    // 复制文件
    fs::copy(&path, &target_path).with_context(|| {
        format!(
            "Failed to copy {} to {}",
            path.display(),
            target_path.display()
        )
    })?;

    println!("Script added successfully: {}", target_path.display());

    Ok(())
}
pub fn remove(name: String, yes: bool, extension: Option<String>) -> Result<()> {
    let script_path = get_script_path()?;
    let scripts = load_scripts(script_path)?;

    // 查找匹配的脚本
    let script = scripts
        .into_iter()
        .find(|s| {
            s.name == name && (extension.is_none() || extension.as_ref() == Some(&s.extension))
        })
        .ok_or_else(|| {
            if let Some(ext) = &extension {
                anyhow::anyhow!("Script '{}.{}' not found", name, ext)
            } else {
                anyhow::anyhow!("Script '{}' not found", name)
            }
        })?;

    // 确认删除
    if !yes {
        let display_name = if script.extension.is_empty() {
            script.name.clone()
        } else {
            format!("{}.{}", script.name, script.extension)
        };

        let message = format!("Do you want to delete '{}'?", display_name);
        if !prompt_confirm(&message, false)? {
            println!("Operation cancelled");
            return Ok(());
        }
    }

    // 删除文件
    fs::remove_file(&script.path)
        .with_context(|| format!("Failed to delete script: {}", script.path.display()))?;

    let display_name = if script.extension.is_empty() {
        script.name
    } else {
        format!("{}.{}", script.name, script.extension)
    };
    println!("Script '{}' deleted successfully", display_name);

    Ok(())
}
pub fn new(name: String) -> Result<()> {
    let script_path = get_script_path()?;
    let path = script_path.join(name.clone());
    if path.exists() {
        let message = format!("Script '{}' already exists. Do you want to open it?", name);
        if !prompt_confirm(&message, true)? {
            println!("Operation cancelled");
            return Ok(());
        }
    } else {
        fs::File::create(&path)
            .with_context(|| format!("Failed to create script: {}", path.display()))?;
        println!("Created new script:{}", name);
    }
    let editor = env::var("EDITOR").unwrap_or_else(|_| {
        if cfg!(windows) {
            String::from("notepad")
        } else {
            String::from("nano")
        }
    });
    println!("Opening '{}' in {}...", name, editor);
    CommandExecutor::new(&editor)
        .arg(path.to_string_lossy().to_string())
        .execute()?;
    Ok(())
}
pub fn edit(name: String, extension: Option<String>) -> Result<()> {
    let script_path = get_script_path()?;
    let scripts = load_scripts(script_path)?;
    let script = scripts
        .into_iter()
        .find(|s| {
            s.name == name && (extension.is_none() || extension.as_ref() == Some(&s.extension))
        })
        .ok_or_else(|| {
            if let Some(ext) = &extension {
                anyhow::anyhow!("Script '{}.{}' not found", name, ext)
            } else {
                anyhow::anyhow!("Script '{}' not found", name)
            }
        })?;
    let editor = env::var("EDITOR").unwrap_or_else(|_| {
        if cfg!(windows) {
            String::from("notepad")
        } else {
            String::from("nano")
        }
    });
    println!("Opening '{}' in {}...", name, editor);
    CommandExecutor::new(&editor)
        .arg(script.path.to_string_lossy().to_string())
        .execute()?;
    Ok(())
}
