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

    // 根据文件扩展名自动检测格式，并提供智能回退（不区分大小写）
    let extension = path.extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase());

    let plugin = match extension.as_deref() {
        Some("toml") => {
            // 尝试解析为 TOML，如果失败则检查是否实际上是 YAML
            Plugin::from_toml(&content).or_else(|toml_err| {
                if let Ok(yaml_plugin) = Plugin::from_yaml(&content) {
                    eprintln!("警告: 文件扩展名为 .toml，但内容看起来是 YAML 格式");
                    eprintln!("已成功解析为 YAML，但建议将文件重命名为 .yaml 或 .yml 扩展名");
                    Ok(yaml_plugin)
                } else {
                    Err(toml_err).context(format!(
                        "无法解析插件文件 '{}' - 扩展名为 .toml 但内容既不是有效的 TOML 也不是 YAML",
                        path.display()
                    ))
                }
            })?
        }
        Some("yaml") | Some("yml") => {
            // 尝试解析为 YAML，如果失败则检查是否实际上是 TOML
            Plugin::from_yaml(&content).or_else(|yaml_err| {
                if let Ok(toml_plugin) = Plugin::from_toml(&content) {
                    eprintln!("警告: 文件扩展名为 .yaml/.yml，但内容看起来是 TOML 格式");
                    eprintln!("已成功解析为 TOML，但建议将文件重命名为 .toml 扩展名");
                    Ok(toml_plugin)
                } else {
                    Err(yaml_err).context(format!(
                        "无法解析插件文件 '{}' - 扩展名为 .yaml/.yml 但内容既不是有效的 YAML 也不是 TOML",
                        path.display()
                    ))
                }
            })?
        }
        _ => {
            // 如果扩展名无法识别，尝试两种格式
            Plugin::from_yaml(&content)
                .or_else(|yaml_err| {
                    Plugin::from_toml(&content).map_err(|toml_err| {
                        anyhow::anyhow!(
                            "无法解析插件文件 '{}' - 请确保文件是有效的 YAML 或 TOML 格式\n  YAML 错误: {}\n  TOML 错误: {}",
                            path.display(),
                            yaml_err,
                            toml_err
                        )
                    })
                })
                .context("建议使用 .yaml、.yml 或 .toml 文件扩展名")?
        }
    };

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
pub fn export(name: String, output: Option<PathBuf>, format: Option<String>) -> Result<()> {
    let db = PluginDatabase::load()?;
    let plugin = db
        .get_plugin(&name)
        .ok_or_else(|| anyhow!("Plugin '{}' not found", name))?;

    // 确定输出格式
    let output_format = if let Some(fmt) = format {
        fmt
    } else if let Some(ref path) = output {
        // 根据输出文件扩展名自动检测格式（不区分大小写）
        let ext = path.extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase());
        match ext.as_deref() {
            Some("toml") => "toml".to_string(),
            Some("yaml") | Some("yml") => "yaml".to_string(),
            _ => "yaml".to_string(), // 默认使用 YAML
        }
    } else {
        "yaml".to_string() // 默认使用 YAML
    };

    // 根据格式序列化插件
    let content = match output_format.as_str() {
        "toml" => plugin.to_toml()?,
        "yaml" => plugin.to_yaml()?,
        _ => bail!("Unsupported format: {}", output_format),
    };

    if let Some(output_path) = output {
        fs::write(&output_path, content)
            .with_context(|| format!("Failed to write to {:?}", output_path))?;
        println!("Plugin '{}' exported to: {:?} (format: {})", name, output_path, output_format);
    } else {
        println!("{}", content);
    }
    Ok(())
}
