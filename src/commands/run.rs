use std::io::{self, Write};
use std::path::Path;

use anyhow::Result;

use crate::{
    core::{
        executor::CommandExecutor,
        plugin::{Plugin, PluginDatabase},
        script::{Script, load_scripts},
    },
    utils::fs::get_script_path,
};

pub fn run(
    name: String,
    extension: Option<String>,
    plugin_name: Option<String>,
    args: Vec<String>,
) -> Result<()> {
    // 1. 查找脚本
    let script = find_script(&name, extension.as_deref())?;

    // 2. 加载插件数据库
    let db = PluginDatabase::load()?;

    // 3. 根据是否指定插件名称，选择不同的加载方式
    let plugin = if let Some(name) = plugin_name {
        // 用户指定了插件名称，加载并验证是否支持该扩展名
        let plugin = load_plugin_by_name(&db, &name)?;

        // 验证插件是否支持该脚本的扩展名
        if !plugin.extensions.contains(&script.extension) {
            anyhow::bail!(
                "Plugin '{}' does not support '.{}' extension.\nSupported extensions: {}",
                plugin.name,
                script.extension,
                plugin.extensions.join(", ")
            );
        }

        plugin
    } else {
        // 未指定插件，根据扩展名查找（可能需要交互选择）
        load_plugin_for_extension(&db, &script.extension)?
    };

    // 4. 构建命令参数
    let cmd_args = build_command_args(&plugin, &script.path, &args)?;

    // 5. 执行脚本
    CommandExecutor::new(&plugin.executor)
        .args(cmd_args)
        .execute()?;

    Ok(())
}

/// 查找脚本
fn find_script(name: &str, extension: Option<&str>) -> Result<Script> {
    let script_path = get_script_path()?;
    let scripts = load_scripts(script_path)?;

    scripts
        .into_iter()
        .find(|s| s.name == name && extension.map_or(true, |ext| s.extension == ext))
        .ok_or_else(|| match extension {
            Some(ext) => anyhow::anyhow!("Script '{}.{}' not found", name, ext),
            None => anyhow::anyhow!("Script '{}' not found", name),
        })
}

/// 根据插件名称加载插件
fn load_plugin_by_name(db: &PluginDatabase, name: &str) -> Result<Plugin> {
    db.get_plugin(name).cloned().ok_or_else(|| {
        anyhow::anyhow!(
            "Plugin '{}' not found.\n\
                Use 'rune plugin list' to see available plugins.",
            name
        )
    })
}

/// 根据扩展名查找插件，如果有多个匹配则提示用户选择
fn load_plugin_for_extension(db: &PluginDatabase, extension: &str) -> Result<Plugin> {
    let matches = db.find_all_by_extension(extension);

    match matches.len() {
        0 => {
            anyhow::bail!(
                "No plugin found for extension '.{}'.\n\
                Use 'rune plugin list' to see available plugins or \
                'rune plugin add <source>' to add a new plugin.",
                extension
            )
        }
        1 => {
            // 只有一个匹配，直接使用
            Ok(matches[0].clone())
        }
        _ => {
            // 多个匹配，提示用户选择
            select_plugin_interactive(&matches, extension)
        }
    }
}

/// 交互式选择插件
fn select_plugin_interactive(plugins: &[&Plugin], extension: &str) -> Result<Plugin> {
    println!("\n⚠️  Multiple plugins support '.{}' extension:", extension);
    println!();

    for (i, plugin) in plugins.iter().enumerate() {
        println!("  [{}] {} ({})", i + 1, plugin.name, plugin.executor);
        if !plugin.description.is_empty() {
            println!("      {}", plugin.description);
        }
    }

    println!();
    print!("Select a plugin [1-{}]: ", plugins.len());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let selection = input
        .trim()
        .parse::<usize>()
        .ok()
        .and_then(|n| {
            if n > 0 && n <= plugins.len() {
                Some(n - 1)
            } else {
                None
            }
        })
        .ok_or_else(|| anyhow::anyhow!("Invalid selection"))?;

    let selected = plugins[selection];
    println!("✓ Using plugin: {}\n", selected.name);

    Ok(selected.clone())
}

/// 构建命令参数
fn build_command_args(
    plugin: &Plugin,
    script_path: &Path,
    user_args: &[String],
) -> Result<Vec<String>> {
    let script_path_str = script_path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid script path: non-UTF8 characters"))?;

    let mut cmd_args = Vec::new();

    // 处理参数模板
    for template in &plugin.arg_template {
        let arg = template.replace("{file}", script_path_str);
        cmd_args.push(arg);
    }

    // 添加用户参数
    cmd_args.extend(user_args.iter().cloned());

    Ok(cmd_args)
}
