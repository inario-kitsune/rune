use std::{fs, path::PathBuf};

use anyhow::{Context, Result, anyhow, bail};
use tabled::{Table, Tabled, settings::Style};

use crate::{
    core::plugin::{Plugin, PluginDatabase},
    utils::cli::prompt_confirm,
};

#[derive(Debug, Tabled)]
struct PluginListInfo {
    #[tabled(rename = "Index")]
    index: usize,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Extension")]
    extension: String,
}

pub fn add(path: PathBuf, force: bool) -> Result<()> {
    // 检查源文件是否存在
    if !path.exists() {
        bail!("Target plugin does not exist:{}", path.display());
    }
    // 检查是否是文件
    if !path.is_file() {
        bail!("Target path is not a file: {}", path.display());
    }
    let content = fs::read_to_string(&path)?;
    let plugin = Plugin::from_yaml(&content)?;
    plugin.validate()?;
    let mut pdb = PluginDatabase::load()?;
    if pdb.get_plugin(&plugin.name).is_some() {
        if !force {
            let message = format!(
                "Plugin '{}' already exists. Do you want to overwrite it?",
                plugin.name
            );
            if !prompt_confirm(&message, false)? {
                println!("Operation cancelled");
                return Ok(());
            }
        }
        // 更新插件
        pdb.update_plugin(plugin.clone())?;
        println!("Plugin '{}' updated successfully", plugin.name);
    } else {
        // 添加新插件
        pdb.add_plugin(plugin.clone())?;
        println!("Plugin '{}' added successfully", plugin.name);
    }
    println!("  Executor: {}", plugin.executor);
    println!("  Extensions: {}", plugin.extensions.join(", "));
    pdb.save()?;
    Ok(())
}
pub fn remove(name: String, yes: bool) -> Result<()> {
    let mut pdb = PluginDatabase::load()?;
    // 检查插件是否存在
    if pdb.get_plugin(&name).is_none() {
        anyhow::bail!("Plugin '{}' not found", name);
    }
    // 确认删除
    if !yes {
        let message = format!("Do you want to remove plugin '{}'?", name);
        if !prompt_confirm(&message, false)? {
            println!("Operation cancelled");
            return Ok(());
        }
    }

    // 删除插件
    pdb.remove_plugin(&name)?;
    println!("Plugin '{}' removed successfully", name);

    // 保存数据库
    pdb.save()?;

    Ok(())
}
pub fn list(plain: bool) -> Result<()> {
    let pdb = PluginDatabase::load()?;
    let plugins: Vec<_> = pdb.all_plugins().collect();
    if plugins.is_empty() {
        println!("No plugins installed");
        return Ok(());
    }
    if plain {
        for plugin in plugins {
            println!("{}", plugin.name);
        }
    } else {
        let scripts: Vec<PluginListInfo> = plugins
            .iter()
            .enumerate()
            .filter_map(|(index, entry)| {
                Some(PluginListInfo {
                    index,
                    name: entry.name.clone(),
                    extension: format!("[{}]", entry.extensions.join(",").clone()),
                })
            })
            .collect();
        let mut table = Table::new(scripts);
        table.with(Style::rounded());
        println!("{}", table);
    };
    Ok(())
}
/// 显示插件详细信息
pub fn info(name: String) -> Result<()> {
    let db = PluginDatabase::load()?;

    let plugin = db
        .get_plugin(&name)
        .ok_or_else(|| anyhow::anyhow!("Plugin '{}' not found", name))?;
    println!("Plugin: {}", plugin.name);
    println!("Version: {}", plugin.version);
    if !plugin.author.is_empty() {
        println!("Author: {}", plugin.author);
    }
    if !plugin.description.is_empty() {
        println!("Description: {}", plugin.description);
    }
    println!();
    println!("Executor: {}", plugin.executor);
    println!("Extensions: {}", plugin.extensions.join(", "));
    println!("Argument Template: {}", plugin.arg_template.join(" "));

    if !plugin.requires.is_empty() {
        println!();
        println!("Requirements:");
        for req in &plugin.requires {
            let available = which::which(req).is_ok();
            let status = if available { "✓" } else { "✗" };
            let status_text = if available { "installed" } else { "not found" };
            println!("  {} {} ({})", status, req, status_text);
        }
    }

    // 检查执行器是否可用
    println!();
    let executor_available = which::which(&plugin.executor).is_ok();
    if executor_available {
        println!("Status: ✓ Ready to use");
    } else {
        println!("Status: ✗ Executor not found in PATH");
        println!(
            "        Please install '{}' to use this plugin",
            plugin.executor
        );
    }

    Ok(())
}
pub fn export(name: String, output: Option<PathBuf>) -> Result<()> {
    let db = PluginDatabase::load()?;
    let plugin = db
        .get_plugin(&name)
        .ok_or_else(|| anyhow!("Plugin '{}' not found", name))?;
    let yaml = plugin.to_yaml()?;
    if let Some(output_path) = output {
        fs::write(&output_path, yaml)
            .with_context(|| format!("Failed to write to {:?}", output_path))?;
        println!("Plugin '{}' exported to: {:?}", name, output_path);
    } else {
        println!("{}", yaml);
    }
    Ok(())
}
