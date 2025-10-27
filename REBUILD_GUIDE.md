# Rune 项目重建指南

> 本文档提供了使用其他编程语言重建 Rune 项目所需的完整架构、设计决策和实现细节。

## 目录

1. [项目概述](#项目概述)
2. [核心架构](#核心架构)
3. [数据模型](#数据模型)
4. [关键算法](#关键算法)
5. [模块详细设计](#模块详细设计)
6. [外部依赖需求](#外部依赖需求)
7. [文件系统布局](#文件系统布局)
8. [测试策略](#测试策略)
9. [实现路线图](#实现路线图)

---

## 项目概述

### 项目简介

**Rune** 是一个通用脚本运行器,允许用户在一个集中的仓库中管理和执行多种编程语言的脚本。

### 核心功能

1. **集中式脚本存储** - 所有脚本存储在单一位置 (`~/.local/share/rune/scripts`)
2. **语言无关执行** - 通过插件系统支持任意编程语言
3. **自动插件检测** - 根据文件扩展名自动匹配合适的执行器
4. **交互式插件选择** - 当多个插件支持同一扩展名时,提示用户选择
5. **脚本管理** - 添加、删除、列表、创建、编辑脚本
6. **插件管理** - 添加、删除、列表、查看信息、导出插件定义
7. **参数传递** - 脚本可以接收命令行参数
8. **跨平台支持** - Linux/macOS/Windows

### 使用场景示例

```bash
# 运行脚本
rune run backup-db -- production 2024-01-15

# 添加新脚本
rune script add ~/scripts/process-data.py

# 创建新脚本
rune script new analyze-logs

# 管理插件
rune plugin add python.yaml
rune plugin info python
rune plugin list
```

---

## 核心架构

### 分层架构

```
┌─────────────────────────────────────────┐
│       CLI Interface (clap)              │
│   命令解析、参数验证、帮助生成           │
└──────────────────┬──────────────────────┘
                   │
┌──────────────────▼──────────────────────┐
│     Command Handlers (commands/)        │
│   run.rs | script.rs | plugin.rs       │
│   业务逻辑协调、用户交互                 │
└──────────────────┬──────────────────────┘
                   │
┌──────────────────▼──────────────────────┐
│        Core Logic (core/)               │
│   executor.rs | plugin.rs | script.rs  │
│   数据模型、持久化、核心算法             │
└──────────────────┬──────────────────────┘
                   │
┌──────────────────▼──────────────────────┐
│      Utilities (utils/)                 │
│   fs.rs | cli.rs                        │
│   路径解析、用户提示                     │
└─────────────────────────────────────────┘
                   │
┌──────────────────▼──────────────────────┐
│   External Dependencies                 │
│   文件系统 | 进程管理 | 序列化           │
└─────────────────────────────────────────┘
```

### 关键设计决策

| 决策项 | 选择 | 理由 |
|--------|------|------|
| 数据存储格式 | 二进制序列化 (bincode) | 快速、紧凑、版本控制 |
| 插件系统 | 配置而非代码 | 安全、简单、无需编译 |
| 插件查找 | HashMap (名称), 线性扫描 (扩展名) | O(1) 按名称查找,扩展名查找频率低 |
| 进程执行 | 继承 stdin/stdout/stderr | 支持交互式程序、实时输出 |
| 错误处理 | 带上下文的异常传播 | 清晰的错误消息便于调试 |
| CLI 框架 | 声明式派生 | 减少样板代码,自动生成帮助 |

---

## 数据模型

### 核心数据结构

#### 1. Plugin (插件)

```rust
struct Plugin {
    name: String,              // 唯一标识符,如 "python"
    executor: String,          // 执行器命令,如 "python3"
    arg_template: Vec<String>, // 参数模板,如 ["-u", "{file}"]
    extensions: Vec<String>,   // 支持的扩展名,如 ["py", "pyw"]
    description: String,       // 可选的描述信息
    author: String,           // 作者
    version: String,          // 版本号
    requires: Vec<String>,    // 依赖的命令列表
}
```

**字段说明:**

- `name`: 插件的唯一标识,不能重复
- `executor`: 系统中可执行的命令名称,必须在 PATH 中
- `arg_template`: 执行时的参数模板,`{file}` 会被替换为实际脚本路径
- `extensions`: 该插件支持的文件扩展名数组
- `requires`: 可选依赖检查列表 (仅警告,不阻止)

**YAML 格式示例:**

```yaml
name: python
executor: python3
arg_template:
  - "-u"      # 无缓冲模式
  - "{file}"  # 脚本路径占位符
extensions:
  - py
  - pyw
description: "Python 3 script executor"
author: "Rune Team"
version: "1.0.0"
requires:
  - pip3
```

#### 2. PluginDatabase (插件数据库)

```rust
struct PluginDatabase {
    plugins: HashMap<String, Plugin>,  // 插件名 -> 插件对象
    version: u32,                      // 数据库版本号
}
```

**方法列表:**

- `load()` - 从磁盘加载数据库,不存在则创建
- `save()` - 序列化并保存到磁盘
- `add_plugin(plugin)` - 添加新插件 (不覆盖)
- `update_plugin(plugin)` - 更新现有插件 (覆盖)
- `remove_plugin(name)` - 删除插件
- `get_plugin(name)` - 按名称查找
- `find_all_by_extension(ext)` - 查找支持指定扩展名的所有插件
- `all_plugins()` - 返回所有插件

**持久化机制:**

- 格式: 二进制序列化 (使用 bincode 或等效库)
- 路径: `~/.local/share/rune/plugin/plugin.dat` (Linux/macOS)
- 路径: `%APPDATA%\rune\plugin\plugin.dat` (Windows)

#### 3. Script (脚本)

```rust
struct Script {
    name: String,       // 脚本名称 (不含扩展名),如 "backup"
    extension: String,  // 文件扩展名,如 "sh"
    path: PathBuf,      // 完整的绝对路径
}
```

**加载机制:**

- 扫描脚本目录 (`~/.local/share/rune/scripts`)
- 仅包含文件 (忽略子目录)
- 必须有扩展名 (忽略 Makefile 等无扩展名文件)
- 从文件名提取名称和扩展名

#### 4. CommandExecutor (命令执行器)

```rust
struct CommandExecutor {
    command: String,     // 执行器命令,如 "python3"
    args: Vec<String>,   // 参数列表
}
```

**方法:**

- `new(command)` - 创建执行器
- `arg(arg)` - 添加单个参数 (返回 self,支持链式调用)
- `args(args)` - 添加多个参数
- `run()` - 执行命令,等待完成,检查退出码

**执行特性:**

- 继承父进程的 stdin/stdout/stderr
- 继承环境变量和工作目录
- 非零退出码视为错误

---

## 关键算法

### 算法 1: 脚本执行流程

```
输入: script_name, optional_extension, optional_plugin_name, optional_args

步骤:
1. 目录扫描
   - 读取脚本目录中的所有文件
   - 过滤: 仅保留有扩展名的文件
   - 提取: 文件名 (不含扩展名) 和扩展名

2. 脚本匹配
   - 查找 script.name == input_name 的脚本
   - 如果指定了 extension, 同时匹配 script.extension
   - 未找到则返回错误

3. 插件选择
   - 从磁盘加载插件数据库
   - 如果指定了 plugin_name:
     a. 按名称获取插件
     b. 验证插件支持该脚本的扩展名
   - 否则:
     a. 查找所有支持该扩展名的插件
     b. 0 个匹配: 错误 "没有支持该扩展名的插件"
     c. 1 个匹配: 使用它
     d. 2+ 个匹配: 交互式选择 (显示编号列表,用户输入选择)

4. 参数构建
   - 遍历 plugin.arg_template:
     * 如果包含 "{file}", 替换为实际脚本路径
     * 否则保持原样
   - 追加用户提供的参数

5. 执行
   - 检查执行器是否在 PATH 中
   - 生成进程: executor + 构建的参数
   - 继承 stdin/stdout/stderr (支持交互)
   - 等待完成
   - 检查退出码 (非零则失败)

输出: 成功或带上下文的错误信息
```

**示例执行流程:**

```
命令: rune run backup -- production 2024-01-15

1. 扫描 ~/.local/share/rune/scripts/
   发现: backup.sh, backup.py, deploy.sh

2. 匹配: backup.sh (name="backup", ext="sh")

3. 加载插件数据库
   查找支持 "sh" 扩展名的插件
   发现: bash 插件

4. 构建参数:
   bash.arg_template = ["{file}"]
   替换后: ["/home/user/.local/share/rune/scripts/backup.sh"]
   追加用户参数: ["production", "2024-01-15"]

5. 执行:
   命令: bash
   参数: ["/home/user/.local/share/rune/scripts/backup.sh", "production", "2024-01-15"]
   结果: 继承当前 shell 的输入输出
```

### 算法 2: 交互式插件选择

```
输入: plugins: Vec<Plugin>, extension: String

步骤:
1. 显示标题: "Multiple plugins support '.ext' extension:"

2. 枚举插件 (从 1 开始):
   [1] python (python3)
       Description: Python 3 script executor
   [2] python2 (python2.7)
       Description: Legacy Python 2 support

3. 提示: "Select a plugin [1-N]: "

4. 解析输入:
   - 必须是 1 到 N 之间的数字
   - 映射到 0 基索引
   - 返回选中的插件

5. 确认: "Using plugin: python"

错误处理:
- 非数字输入: "Invalid input, please enter a number"
- 超出范围: "Invalid selection, please choose 1-N"
```

### 算法 3: 插件数据库持久化

```
加载 (load):
1. 检查 plugin.dat 是否存在
2. 不存在:
   - 创建空数据库 (version=1, plugins={})
   - 保存到磁盘
3. 存在:
   - 读取文件内容
   - 反序列化 (bincode)
   - 验证版本号匹配
4. 返回 PluginDatabase 对象

保存 (save):
1. 序列化数据库为字节流 (bincode)
2. 如需要则创建父目录
3. 原子性写入 plugin.dat

错误处理:
- 文件损坏: 返回描述性错误
- 版本不匹配: 提示需要升级
- 权限问题: 显示路径和权限错误
```

---

## 模块详细设计

### 模块 1: CLI Interface (命令行接口)

**文件:** `src/commands/mod.rs`

**职责:** 定义命令行结构、参数、帮助信息

**数据结构:**

```rust
struct Cli {
    command: Commands,
}

enum Commands {
    Run {
        name: String,              // 脚本名称
        extension: Option<String>, // 可选: 指定扩展名
        plugin: Option<String>,    // 可选: 指定插件
        args: Vec<String>,         // 传递给脚本的参数
    },
    Script {
        command: ScriptCommands,
    },
    Plugin {
        command: PluginCommands,
    },
}

enum ScriptCommands {
    Add {
        path: PathBuf,           // 要添加的脚本路径
        force: Option<bool>,     // 是否覆盖
    },
    Remove {
        name: String,            // 脚本名称
        yes: Option<bool>,       // 跳过确认
        extension: Option<String>, // 指定扩展名
    },
    List {
        plain: Option<bool>,     // 纯文本输出
    },
    New {
        name: String,            // 新脚本名称
    },
    Edit {
        name: String,            // 脚本名称
        extension: Option<String>,
    },
}

enum PluginCommands {
    Add {
        path: PathBuf,           // YAML 文件路径
        force: Option<bool>,
    },
    Remove {
        name: String,            // 插件名称
        yes: Option<bool>,
    },
    List {
        plain: Option<bool>,
    },
    Info {
        name: String,            // 插件名称
    },
    Export {
        name: String,
        output: Option<PathBuf>, // 输出路径
    },
}
```

**命令映射:**

| 命令 | 别名 | 说明 |
|------|------|------|
| `rune run` | `r` | 运行脚本 |
| `rune script` | `s` | 管理脚本 |
| `rune plugin` | `p` | 管理插件 |

**参数分隔符:** 使用 `--` 分隔 rune 参数和脚本参数

```bash
rune run backup -- --force --output=/tmp
           ↑       ↑  ↑
      脚本名称  分隔符  传给脚本的参数
```

### 模块 2: Run Command (运行命令)

**文件:** `src/commands/run.rs`

**核心函数:**

```rust
fn run(
    name: String,
    extension: Option<String>,
    plugin_name: Option<String>,
    args: Vec<String>
) -> Result<()>
```

**实现步骤:**

```rust
// 1. 查找脚本
let script_path = get_script_path()?;
let scripts = load_scripts(&script_path)?;
let script = find_script(&scripts, &name, extension)?;

// 2. 加载插件
let plugin_db = PluginDatabase::load()?;
let plugin = if let Some(name) = plugin_name {
    load_plugin_by_name(&plugin_db, &name, &script.extension)?
} else {
    load_plugin_for_extension(&plugin_db, &script.extension)?
};

// 3. 构建参数
let command_args = build_command_args(&plugin, &script.path, args)?;

// 4. 执行
let executor = CommandExecutor::new(&plugin.executor)
    .args(&command_args);
executor.run()?;

Ok(())
```

**辅助函数:**

```rust
fn find_script(scripts: &[Script], name: &str, ext: Option<String>) -> Result<Script>
// 在脚本列表中查找匹配的脚本

fn load_plugin_by_name(db: &PluginDatabase, name: &str, ext: &str) -> Result<Plugin>
// 按名称加载插件并验证支持指定扩展名

fn load_plugin_for_extension(db: &PluginDatabase, ext: &str) -> Result<Plugin>
// 查找支持指定扩展名的插件,多个则交互选择

fn select_plugin_interactive(plugins: Vec<&Plugin>, ext: &str) -> Result<Plugin>
// 显示编号菜单,让用户选择插件

fn build_command_args(plugin: &Plugin, script_path: &Path, args: Vec<String>) -> Result<Vec<String>>
// 处理 arg_template,替换 {file},追加用户参数
```

### 模块 3: Script Management (脚本管理)

**文件:** `src/commands/script.rs`

**主要功能实现:**

```rust
// 列出所有脚本
fn list(plain: bool) -> Result<()> {
    let scripts = load_scripts(&get_script_path()?)?;

    if plain {
        for script in scripts {
            println!("{}.{}", script.name, script.extension);
        }
    } else {
        // 使用表格格式显示
        let table = scripts.iter()
            .enumerate()
            .map(|(i, s)| ScriptListInfo {
                index: i + 1,
                name: s.name.clone(),
                extension: s.extension.clone(),
            })
            .collect::<Vec<_>>();

        println!("{}", create_table(table));
    }

    Ok(())
}

// 添加脚本
fn add(path: PathBuf, force: bool) -> Result<()> {
    let script_dir = get_script_path()?;
    let filename = path.file_name().ok_or("Invalid path")?;
    let dest = script_dir.join(filename);

    // 检查是否已存在
    if dest.exists() && !force {
        if !prompt_confirm("Script exists. Overwrite?", false)? {
            return Ok(());
        }
    }

    std::fs::copy(&path, &dest)?;
    println!("Added script: {}", filename);
    Ok(())
}

// 删除脚本
fn remove(name: String, yes: bool, extension: Option<String>) -> Result<()> {
    let scripts = load_scripts(&get_script_path()?)?;
    let script = find_script(&scripts, &name, extension)?;

    if !yes {
        let prompt = format!("Remove script '{}.{}'?", script.name, script.extension);
        if !prompt_confirm(&prompt, false)? {
            return Ok(());
        }
    }

    std::fs::remove_file(&script.path)?;
    println!("Removed script: {}.{}", script.name, script.extension);
    Ok(())
}

// 创建新脚本
fn new(name: String) -> Result<()> {
    let script_path = get_script_path()?.join(&name);

    // 创建空文件
    std::fs::write(&script_path, "")?;

    // 在编辑器中打开
    open_in_editor(&script_path)?;
    Ok(())
}

// 编辑脚本
fn edit(name: String, extension: Option<String>) -> Result<()> {
    let scripts = load_scripts(&get_script_path()?)?;
    let script = find_script(&scripts, &name, extension)?;

    open_in_editor(&script.path)?;
    Ok(())
}

// 辅助: 在编辑器中打开文件
fn open_in_editor(path: &Path) -> Result<()> {
    let editor = std::env::var("EDITOR")
        .unwrap_or_else(|_| {
            if cfg!(windows) { "notepad" } else { "nano" }
                .to_string()
        });

    CommandExecutor::new(&editor)
        .arg(path.to_str().unwrap())
        .run()?;

    Ok(())
}
```

### 模块 4: Plugin Management (插件管理)

**文件:** `src/commands/plugin.rs`

**主要功能实现:**

```rust
// 添加插件
fn add(path: PathBuf, force: bool) -> Result<()> {
    // 1. 读取并解析 YAML
    let yaml_content = std::fs::read_to_string(&path)?;
    let plugin = Plugin::from_yaml(&yaml_content)?;

    // 2. 验证插件
    plugin.validate(); // 仅警告,不阻止

    // 3. 加载数据库
    let mut db = PluginDatabase::load()?;

    // 4. 检查重复
    if db.get_plugin(&plugin.name).is_some() && !force {
        if !prompt_confirm("Plugin exists. Overwrite?", false)? {
            return Ok(());
        }
        db.update_plugin(plugin)?;
    } else {
        db.add_plugin(plugin.clone())?;
    }

    // 5. 保存
    db.save()?;

    println!("Added plugin: {}", plugin.name);
    Ok(())
}

// 删除插件
fn remove(name: String, yes: bool) -> Result<()> {
    let mut db = PluginDatabase::load()?;

    if !yes {
        let prompt = format!("Remove plugin '{}'?", name);
        if !prompt_confirm(&prompt, false)? {
            return Ok(());
        }
    }

    db.remove_plugin(&name)?;
    db.save()?;

    println!("Removed plugin: {}", name);
    Ok(())
}

// 列出插件
fn list(plain: bool) -> Result<()> {
    let db = PluginDatabase::load()?;
    let plugins = db.all_plugins();

    if plain {
        for plugin in plugins {
            println!("{}", plugin.name);
        }
    } else {
        // 表格格式
        let table = plugins.enumerate()
            .map(|(i, p)| PluginListInfo {
                index: i + 1,
                name: p.name.clone(),
                extensions: p.extensions.join(", "),
            })
            .collect::<Vec<_>>();

        println!("{}", create_table(table));
    }

    Ok(())
}

// 查看插件信息
fn info(name: String) -> Result<()> {
    let db = PluginDatabase::load()?;
    let plugin = db.get_plugin(&name)
        .ok_or("Plugin not found")?;

    println!("Name: {}", plugin.name);
    println!("Version: {}", plugin.version);
    println!("Author: {}", plugin.author);
    println!("Description: {}", plugin.description);
    println!("Executor: {}", plugin.executor);
    println!("Extensions: {}", plugin.extensions.join(", "));
    println!("Arg Template: {:?}", plugin.arg_template);

    // 检查依赖
    if !plugin.requires.is_empty() {
        println!("\nDependencies:");
        for dep in &plugin.requires {
            let status = if command_exists(dep) { "✓" } else { "✗" };
            println!("  {} {}", status, dep);
        }
    }

    // 检查执行器
    let executor_status = if command_exists(&plugin.executor) { "✓" } else { "✗" };
    println!("\nExecutor: {} {}", executor_status, plugin.executor);

    Ok(())
}

// 导出插件
fn export(name: String, output: Option<PathBuf>) -> Result<()> {
    let db = PluginDatabase::load()?;
    let plugin = db.get_plugin(&name)
        .ok_or("Plugin not found")?;

    let yaml = plugin.to_yaml()?;

    if let Some(path) = output {
        std::fs::write(&path, yaml)?;
        println!("Exported to: {}", path.display());
    } else {
        println!("{}", yaml);
    }

    Ok(())
}
```

### 模块 5: Core Plugin System (核心插件系统)

**文件:** `src/core/plugin.rs`

**完整实现:**

```rust
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize)]
pub struct Plugin {
    pub name: String,
    pub executor: String,
    #[serde(default = "default_arg_template")]
    pub arg_template: Vec<String>,
    pub extensions: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub requires: Vec<String>,
}

fn default_arg_template() -> Vec<String> {
    vec!["{file}".to_string()]
}

impl Plugin {
    // 从 YAML 解析
    pub fn from_yaml(yaml: &str) -> Result<Self> {
        let plugin: Self = serde_yaml::from_str(yaml)
            .context("Failed to parse plugin YAML")?;

        // 验证必需字段
        if plugin.name.is_empty() {
            return Err(anyhow!("Plugin name is required"));
        }
        if plugin.executor.is_empty() {
            return Err(anyhow!("Executor is required"));
        }
        if plugin.extensions.is_empty() {
            return Err(anyhow!("At least one extension is required"));
        }

        Ok(plugin)
    }

    // 转换为 YAML
    pub fn to_yaml(&self) -> Result<String> {
        serde_yaml::to_string(self)
            .context("Failed to serialize plugin to YAML")
    }

    // 验证插件 (仅警告)
    pub fn validate(&self) {
        // 检查执行器
        if !command_exists(&self.executor) {
            eprintln!("Warning: Executor '{}' not found in PATH", self.executor);
        }

        // 检查依赖
        for dep in &self.requires {
            if !command_exists(dep) {
                eprintln!("Warning: Dependency '{}' not found in PATH", dep);
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PluginDatabase {
    plugins: HashMap<String, Plugin>,
    version: u32,
}

impl PluginDatabase {
    const VERSION: u32 = 1;

    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            version: Self::VERSION,
        }
    }

    // 从磁盘加载
    pub fn load() -> Result<Self> {
        let db_path = get_plugin_db()?;

        if !db_path.exists() {
            let db = Self::new();
            db.save()?;
            return Ok(db);
        }

        let bytes = std::fs::read(&db_path)
            .context("Failed to read plugin database")?;

        let db: Self = bincode::deserialize(&bytes)
            .context("Failed to deserialize plugin database")?;

        if db.version != Self::VERSION {
            return Err(anyhow!(
                "Plugin database version mismatch: expected {}, got {}",
                Self::VERSION,
                db.version
            ));
        }

        Ok(db)
    }

    // 保存到磁盘
    pub fn save(&self) -> Result<()> {
        let db_path = get_plugin_db()?;

        // 确保父目录存在
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let bytes = bincode::serialize(self)
            .context("Failed to serialize plugin database")?;

        std::fs::write(&db_path, bytes)
            .context("Failed to write plugin database")?;

        Ok(())
    }

    // 添加插件 (不覆盖)
    pub fn add_plugin(&mut self, plugin: Plugin) -> Result<()> {
        if self.plugins.contains_key(&plugin.name) {
            return Err(anyhow!("Plugin '{}' already exists", plugin.name));
        }
        self.plugins.insert(plugin.name.clone(), plugin);
        Ok(())
    }

    // 更新插件 (覆盖)
    pub fn update_plugin(&mut self, plugin: Plugin) -> Result<()> {
        self.plugins.insert(plugin.name.clone(), plugin);
        Ok(())
    }

    // 删除插件
    pub fn remove_plugin(&mut self, name: &str) -> Result<()> {
        self.plugins.remove(name)
            .ok_or_else(|| anyhow!("Plugin '{}' not found", name))?;
        Ok(())
    }

    // 按名称获取
    pub fn get_plugin(&self, name: &str) -> Option<&Plugin> {
        self.plugins.get(name)
    }

    // 按扩展名查找所有插件
    pub fn find_all_by_extension(&self, extension: &str) -> Vec<&Plugin> {
        let mut plugins: Vec<&Plugin> = self.plugins.values()
            .filter(|p| p.extensions.iter().any(|e| e == extension))
            .collect();

        // 按名称排序
        plugins.sort_by(|a, b| a.name.cmp(&b.name));

        plugins
    }

    // 获取所有插件
    pub fn all_plugins(&self) -> impl Iterator<Item = &Plugin> {
        self.plugins.values()
    }
}
```

### 模块 6: Script Loading (脚本加载)

**文件:** `src/core/script.rs`

```rust
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Script {
    pub name: String,
    pub extension: String,
    pub path: PathBuf,
}

// 加载目录中的所有脚本
pub fn load_scripts(path: &Path) -> Result<Vec<Script>> {
    if !path.exists() {
        return Err(anyhow!("Script directory does not exist: {}", path.display()));
    }

    let mut scripts = Vec::new();

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        // 仅处理文件
        if !path.is_file() {
            continue;
        }

        // 提取文件名和扩展名
        let filename = match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => continue,
        };

        // 分离名称和扩展名
        let parts: Vec<&str> = filename.rsplitn(2, '.').collect();
        if parts.len() != 2 {
            // 没有扩展名,跳过
            continue;
        }

        let extension = parts[0].to_string();
        let name = parts[1].to_string();

        scripts.push(Script {
            name,
            extension,
            path: path.canonicalize()?, // 转换为绝对路径
        });
    }

    Ok(scripts)
}
```

### 模块 7: Command Executor (命令执行器)

**文件:** `src/core/executor.rs`

```rust
use std::process::Command;

pub struct CommandExecutor {
    command: String,
    args: Vec<String>,
}

impl CommandExecutor {
    pub fn new(command: &str) -> Self {
        Self {
            command: command.to_string(),
            args: Vec::new(),
        }
    }

    // 添加单个参数
    pub fn arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        self
    }

    // 添加多个参数
    pub fn args(mut self, args: &[String]) -> Self {
        self.args.extend_from_slice(args);
        self
    }

    // 执行命令
    pub fn run(&self) -> Result<()> {
        // 检查命令是否存在
        if !command_exists(&self.command) {
            return Err(anyhow!(
                "Command '{}' not found in PATH",
                self.command
            ));
        }

        // 创建进程
        let status = Command::new(&self.command)
            .args(&self.args)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()
            .context(format!("Failed to execute '{}'", self.command))?;

        // 检查退出码
        if !status.success() {
            return Err(anyhow!(
                "Command '{}' failed with exit code: {}",
                self.command,
                status.code().unwrap_or(-1)
            ));
        }

        Ok(())
    }
}

// 检查命令是否在 PATH 中
fn command_exists(command: &str) -> bool {
    which::which(command).is_ok()
}
```

### 模块 8: Utilities (工具函数)

**文件:** `src/utils/fs.rs`

```rust
use std::path::PathBuf;

// 获取脚本目录路径
pub fn get_script_path() -> Result<PathBuf> {
    if let Ok(custom_path) = std::env::var("RUNE_REPO") {
        return Ok(PathBuf::from(custom_path));
    }

    let base = get_data_dir()?;
    Ok(base.join("scripts"))
}

// 获取插件目录路径
pub fn get_plugin_path() -> Result<PathBuf> {
    if let Ok(custom_path) = std::env::var("RUNE_PLUGIN") {
        return Ok(PathBuf::from(custom_path));
    }

    let base = get_data_dir()?;
    Ok(base.join("plugin"))
}

// 获取插件数据库路径
pub fn get_plugin_db() -> Result<PathBuf> {
    let plugin_dir = get_plugin_path()?;
    Ok(plugin_dir.join("plugin.dat"))
}

// 获取数据目录 (平台相关)
fn get_data_dir() -> Result<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        // Windows: %APPDATA%\rune
        let appdata = std::env::var("APPDATA")
            .context("APPDATA environment variable not set")?;
        Ok(PathBuf::from(appdata).join("rune"))
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Linux/macOS: ~/.local/share/rune (XDG Base Directory)
        let home = std::env::var("HOME")
            .context("HOME environment variable not set")?;
        Ok(PathBuf::from(home).join(".local/share/rune"))
    }
}
```

**文件:** `src/utils/cli.rs`

```rust
use std::io::{self, Write};

// 提示用户确认
pub fn prompt_confirm(prompt: &str, default_yes: bool) -> Result<bool> {
    let default_text = if default_yes { "Y/n" } else { "y/N" };
    print!("{} [{}]: ", prompt, default_text);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_lowercase();

    if input.is_empty() {
        return Ok(default_yes);
    }

    Ok(input == "y" || input == "yes")
}
```

---

## 外部依赖需求

### 核心依赖

| 功能 | Rust 依赖 | 说明 | 其他语言替代方案 |
|------|-----------|------|------------------|
| CLI 解析 | `clap = "4.5"` | 命令行参数解析 | Python: `argparse`/`click`, Go: `cobra`, Node: `commander` |
| 错误处理 | `anyhow = "1.0"` | 带上下文的错误 | Python: 内置异常, Go: `errors`, Node: `Error` |
| YAML 解析 | `serde_yaml = "0.9"` | 读写 YAML 文件 | Python: `pyyaml`, Go: `gopkg.in/yaml.v3`, Node: `js-yaml` |
| 序列化 | `serde = "1.0"` | 数据序列化框架 | Python: `pickle`/`msgpack`, Go: `encoding/gob`, Node: `msgpack` |
| 二进制序列化 | `bincode = "2.0"` | 高效二进制格式 | Python: `pickle`, Go: `gob`, Node: `msgpack-lite` |
| 表格输出 | `tabled = "0.20"` | 格式化表格显示 | Python: `tabulate`, Go: `tablewriter`, Node: `cli-table3` |
| PATH 查找 | `which = "8.0"` | 查找可执行文件 | Python: `shutil.which()`, Go: `exec.LookPath()`, Node: `which` |

### 开发依赖 (测试)

| 功能 | Rust 依赖 | 其他语言替代方案 |
|------|-----------|------------------|
| 临时文件 | `tempfile = "3.14"` | Python: `tempfile`, Go: `ioutil.TempDir()`, Node: `tmp` |
| 文件断言 | `assert_fs = "1.1"` | 手动实现或使用测试库 |
| 断言谓词 | `predicates = "3.1"` | 内置断言 |
| 串行测试 | `serial_test = "3.2"` | 测试框架的顺序执行功能 |

### 标准库需求

- **文件系统:** 读写文件、创建目录、遍历目录
- **进程管理:** 生成子进程、继承 I/O、获取退出码
- **环境变量:** 读取 PATH, HOME, EDITOR, 自定义变量
- **路径处理:** 路径拼接、规范化、扩展名提取
- **标准 I/O:** 读取用户输入、打印输出

---

## 文件系统布局

### 运行时目录结构

```
Linux/macOS:
~/.local/share/rune/
├── scripts/                    # 脚本存储目录
│   ├── backup.sh              # 示例: Bash 脚本
│   ├── deploy.py              # 示例: Python 脚本
│   ├── analyze.rb             # 示例: Ruby 脚本
│   └── process-data.js        # 示例: Node.js 脚本
└── plugin/                     # 插件配置目录
    └── plugin.dat             # 二进制插件数据库

Windows:
%APPDATA%\rune\
├── scripts\
│   └── (same as above)
└── plugin\
    └── plugin.dat
```

### 环境变量覆盖

```bash
# 自定义脚本目录
export RUNE_REPO=/path/to/custom/scripts

# 自定义插件目录
export RUNE_PLUGIN=/path/to/custom/plugins

# 编辑器设置
export EDITOR=vim
```

### 插件 YAML 文件位置

- 用户可以在任意位置创建 YAML 文件
- 通过 `rune plugin add /path/to/plugin.yaml` 注册
- 注册后内容存储在二进制数据库中
- YAML 文件可以删除 (已加载到数据库)

---

## 测试策略

### 单元测试覆盖

| 模块 | 测试数量 | 覆盖内容 |
|------|----------|----------|
| `core/executor.rs` | 7 | 命令创建、参数链、PATH 检查、执行成功/失败 |
| `core/plugin.rs` | 20 | YAML 解析、验证、序列化、数据库 CRUD、扩展名查找 |
| `core/script.rs` | 9 | 目录扫描、文件过滤、扩展名提取、错误处理 |

### 测试用例示例

**测试 1: 插件 YAML 解析**

```rust
#[test]
fn test_parse_valid_plugin() {
    let yaml = r#"
name: python
executor: python3
arg_template:
  - "-u"
  - "{file}"
extensions:
  - py
  - pyw
description: "Python 3 executor"
"#;

    let plugin = Plugin::from_yaml(yaml).unwrap();
    assert_eq!(plugin.name, "python");
    assert_eq!(plugin.executor, "python3");
    assert_eq!(plugin.extensions, vec!["py", "pyw"]);
}

#[test]
fn test_parse_missing_name() {
    let yaml = r#"
executor: python3
extensions: [py]
"#;

    let result = Plugin::from_yaml(yaml);
    assert!(result.is_err());
}
```

**测试 2: 脚本加载**

```rust
#[test]
fn test_load_scripts() {
    let temp_dir = TempDir::new().unwrap();
    let script_path = temp_dir.path();

    // 创建测试脚本
    std::fs::write(script_path.join("backup.sh"), "#!/bin/bash").unwrap();
    std::fs::write(script_path.join("deploy.py"), "print('hello')").unwrap();

    let scripts = load_scripts(script_path).unwrap();
    assert_eq!(scripts.len(), 2);

    let names: Vec<String> = scripts.iter().map(|s| s.name.clone()).collect();
    assert!(names.contains(&"backup".to_string()));
    assert!(names.contains(&"deploy".to_string()));
}

#[test]
fn test_ignore_files_without_extension() {
    let temp_dir = TempDir::new().unwrap();
    let script_path = temp_dir.path();

    std::fs::write(script_path.join("Makefile"), "all:").unwrap();
    std::fs::write(script_path.join("test.sh"), "").unwrap();

    let scripts = load_scripts(script_path).unwrap();
    assert_eq!(scripts.len(), 1); // 仅 test.sh
}
```

**测试 3: 插件数据库持久化**

```rust
#[test]
fn test_save_and_load_database() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("plugin.dat");

    // 创建并保存数据库
    let mut db = PluginDatabase::new();
    let plugin = Plugin {
        name: "test".to_string(),
        executor: "bash".to_string(),
        arg_template: vec!["{file}".to_string()],
        extensions: vec!["sh".to_string()],
        ..Default::default()
    };
    db.add_plugin(plugin).unwrap();

    // 序列化并保存
    let bytes = bincode::serialize(&db).unwrap();
    std::fs::write(&db_path, bytes).unwrap();

    // 加载并验证
    let bytes = std::fs::read(&db_path).unwrap();
    let loaded_db: PluginDatabase = bincode::deserialize(&bytes).unwrap();

    assert!(loaded_db.get_plugin("test").is_some());
}
```

### 集成测试场景

1. **完整脚本执行流程**
   - 创建临时脚本目录和插件数据库
   - 添加插件
   - 添加脚本
   - 执行脚本并验证输出

2. **多插件选择**
   - 注册多个支持同一扩展名的插件
   - 模拟用户选择
   - 验证正确的插件被使用

3. **错误处理**
   - 脚本不存在
   - 插件不存在
   - 执行器不在 PATH 中
   - 脚本执行失败

---

## 实现路线图

### 第一阶段: 基础架构 (核心功能)

**目标:** 实现最小可用版本 (MVP)

1. **数据模型** (2-3 天)
   - [ ] 实现 `Plugin` 结构体
   - [ ] 实现 `Script` 结构体
   - [ ] YAML 解析和序列化
   - [ ] 单元测试覆盖

2. **插件数据库** (2-3 天)
   - [ ] 实现 `PluginDatabase` 结构体
   - [ ] HashMap 存储和查找
   - [ ] 二进制序列化 (bincode 或等效)
   - [ ] 文件持久化 (load/save)
   - [ ] 扩展名查找算法
   - [ ] 单元测试

3. **脚本加载** (1-2 天)
   - [ ] 实现 `load_scripts()` 函数
   - [ ] 目录遍历
   - [ ] 文件名解析 (名称/扩展名分离)
   - [ ] 路径规范化
   - [ ] 单元测试

4. **命令执行器** (1-2 天)
   - [ ] 实现 `CommandExecutor`
   - [ ] 进程生成和 I/O 继承
   - [ ] PATH 检查
   - [ ] 退出码验证
   - [ ] 单元测试

5. **路径工具** (1 天)
   - [ ] 实现 `get_script_path()`
   - [ ] 实现 `get_plugin_path()`
   - [ ] 实现 `get_plugin_db()`
   - [ ] XDG/Windows 平台兼容
   - [ ] 环境变量支持

**里程碑 1:** 核心模块完成,所有单元测试通过

---

### 第二阶段: 命令实现

**目标:** 实现所有 CLI 命令

6. **CLI 框架** (1-2 天)
   - [ ] 定义命令结构 (Cli, Commands, 子命令)
   - [ ] 参数和标志定义
   - [ ] 帮助文本
   - [ ] 命令别名

7. **Run 命令** (2-3 天)
   - [ ] 实现 `run()` 主函数
   - [ ] 脚本查找逻辑
   - [ ] 插件选择逻辑
   - [ ] 交互式插件选择
   - [ ] 参数构建 (模板替换)
   - [ ] 集成测试

8. **Script 命令** (3-4 天)
   - [ ] 实现 `list()` - 表格和纯文本输出
   - [ ] 实现 `add()` - 文件复制和覆盖提示
   - [ ] 实现 `remove()` - 删除确认
   - [ ] 实现 `new()` - 创建和编辑器集成
   - [ ] 实现 `edit()` - 编辑器集成
   - [ ] 集成测试

9. **Plugin 命令** (3-4 天)
   - [ ] 实现 `add()` - YAML 加载和验证
   - [ ] 实现 `remove()` - 删除确认
   - [ ] 实现 `list()` - 表格和纯文本输出
   - [ ] 实现 `info()` - 详细信息和依赖检查
   - [ ] 实现 `export()` - YAML 导出
   - [ ] 集成测试

10. **用户交互工具** (1 天)
    - [ ] 实现 `prompt_confirm()`
    - [ ] 实现交互式菜单选择
    - [ ] 错误消息格式化

**里程碑 2:** 所有命令可用,基本功能完整

---

### 第三阶段: 完善和优化

**目标:** 提升用户体验和稳定性

11. **错误处理** (2 天)
    - [ ] 统一错误类型
    - [ ] 添加上下文信息
    - [ ] 用户友好的错误消息
    - [ ] 错误恢复建议

12. **表格输出** (1 天)
    - [ ] 实现表格格式化
    - [ ] 列对齐和边框
    - [ ] 颜色支持 (可选)

13. **文档** (2-3 天)
    - [ ] README 文件
    - [ ] 使用示例
    - [ ] 插件开发指南
    - [ ] API 文档

14. **构建和打包** (1-2 天)
    - [ ] 编译优化
    - [ ] 发布脚本
    - [ ] 安装说明
    - [ ] 跨平台测试

**里程碑 3:** 项目可发布

---

### 第四阶段: 高级特性 (可选)

15. **插件模板** (1-2 天)
    - [ ] 常见语言的内置插件模板
    - [ ] 插件生成命令
    - [ ] 插件验证工具

16. **配置文件** (2-3 天)
    - [ ] 全局配置支持 (rune.toml)
    - [ ] 默认插件设置
    - [ ] 别名定义

17. **Shell 集成** (2-3 天)
    - [ ] Bash/Zsh 自动补全
    - [ ] 脚本名称补全
    - [ ] 插件名称补全

18. **增强功能** (按需)
    - [ ] 环境变量传递
    - [ ] 工作目录设置
    - [ ] 日志记录
    - [ ] 性能监控

---

## 实现细节和注意事项

### 1. 二进制序列化选择

**Rust 使用 bincode,其他语言可选:**

| 语言 | 推荐库 | 特点 |
|------|--------|------|
| Python | `pickle` (内置) | 简单,Python 专用 |
| Python | `msgpack` | 跨语言,高效 |
| Go | `encoding/gob` (内置) | Go 专用,高效 |
| Go | `github.com/vmihailenco/msgpack` | 跨语言 |
| Node.js | `msgpack-lite` | 跨语言,成熟 |
| Java | `java.io.Serializable` | 内置 |
| Java | `MessagePack` | 跨语言 |

**数据库版本控制:**

```python
# Python 示例
class PluginDatabase:
    VERSION = 1

    def save(self, path):
        data = {
            'version': self.VERSION,
            'plugins': self.plugins
        }
        with open(path, 'wb') as f:
            pickle.dump(data, f)

    def load(self, path):
        with open(path, 'rb') as f:
            data = pickle.load(f)

        if data['version'] != self.VERSION:
            raise ValueError(f"Version mismatch: {data['version']}")

        self.plugins = data['plugins']
```

### 2. 进程执行注意事项

**继承 I/O 的重要性:**

- **stdin 继承** - 允许脚本从用户接收输入 (如 `input()`, `read`)
- **stdout 继承** - 实时显示输出,不需要等待进程结束
- **stderr 继承** - 错误信息立即可见

**不同语言的实现:**

```python
# Python
import subprocess

subprocess.run(
    [executor] + args,
    stdin=None,      # 继承
    stdout=None,     # 继承
    stderr=None,     # 继承
    check=True       # 非零退出码抛出异常
)
```

```go
// Go
cmd := exec.Command(executor, args...)
cmd.Stdin = os.Stdin
cmd.Stdout = os.Stdout
cmd.Stderr = os.Stderr

err := cmd.Run()
if err != nil {
    return err
}
```

```javascript
// Node.js
const { spawnSync } = require('child_process');

const result = spawnSync(executor, args, {
    stdio: 'inherit'  // 继承所有 I/O
});

if (result.status !== 0) {
    throw new Error(`Command failed with exit code ${result.status}`);
}
```

### 3. 路径规范化

**关键操作:**

- **绝对路径转换** - 使用 `os.path.abspath()` (Python), `filepath.Abs()` (Go)
- **符号链接解析** - 使用 `os.path.realpath()` (Python), `filepath.EvalSymlinks()` (Go)
- **路径分隔符** - 使用 `os.path.join()` 而非手动拼接

**示例:**

```python
# Python
import os

def canonicalize_path(path):
    """转换为规范化的绝对路径"""
    return os.path.realpath(os.path.abspath(path))
```

### 4. 跨平台兼容性检查表

- [ ] 路径分隔符 (使用库函数,不要硬编码 `/` 或 `\`)
- [ ] 数据目录位置 (XDG vs %APPDATA%)
- [ ] 默认编辑器 (nano vs notepad)
- [ ] 行结束符 (LF vs CRLF - 通常由 OS 自动处理)
- [ ] 可执行文件扩展名 (`.exe` on Windows)
- [ ] PATH 分隔符 (`:` on Unix, `;` on Windows)

### 5. YAML 解析最佳实践

**验证顺序:**

1. 语法检查 (YAML 解析器自动完成)
2. Schema 验证 (必需字段检查)
3. 语义验证 (executor 是否在 PATH 中)
4. 业务规则验证 (名称是否重复)

**Python 示例:**

```python
import yaml
from typing import List, Optional

class Plugin:
    def __init__(self, data: dict):
        # 必需字段
        self.name = data['name']
        self.executor = data['executor']
        self.extensions = data['extensions']

        # 可选字段
        self.arg_template = data.get('arg_template', ['{file}'])
        self.description = data.get('description', '')
        self.author = data.get('author', '')
        self.version = data.get('version', '')
        self.requires = data.get('requires', [])

        self.validate()

    def validate(self):
        """验证字段"""
        if not self.name:
            raise ValueError("name is required")
        if not self.executor:
            raise ValueError("executor is required")
        if not self.extensions:
            raise ValueError("at least one extension is required")

    @classmethod
    def from_yaml(cls, yaml_str: str):
        data = yaml.safe_load(yaml_str)
        return cls(data)

    def to_yaml(self) -> str:
        data = {
            'name': self.name,
            'executor': self.executor,
            'arg_template': self.arg_template,
            'extensions': self.extensions,
        }

        # 仅包含非空可选字段
        if self.description:
            data['description'] = self.description
        if self.author:
            data['author'] = self.author
        if self.version:
            data['version'] = self.version
        if self.requires:
            data['requires'] = self.requires

        return yaml.dump(data, default_flow_style=False)
```

### 6. 交互式提示设计

**用户选择插件示例:**

```
Multiple plugins support '.py' extension:

[1] python (python3)
    Python 3 script executor

[2] python2 (python2.7)
    Legacy Python 2 support

[3] uv (uv run)
    Fast Python package runner

Select a plugin [1-3]: 1

Using plugin: python
```

**实现伪代码:**

```python
def select_plugin_interactive(plugins: List[Plugin], extension: str) -> Plugin:
    print(f"\nMultiple plugins support '.{extension}' extension:\n")

    for i, plugin in enumerate(plugins, start=1):
        print(f"[{i}] {plugin.name} ({plugin.executor})")
        if plugin.description:
            print(f"    {plugin.description}")
        print()

    while True:
        try:
            choice = input(f"Select a plugin [1-{len(plugins)}]: ").strip()
            index = int(choice) - 1

            if 0 <= index < len(plugins):
                selected = plugins[index]
                print(f"\nUsing plugin: {selected.name}\n")
                return selected
            else:
                print(f"Invalid selection. Please choose 1-{len(plugins)}.")
        except (ValueError, KeyboardInterrupt):
            print("Invalid input. Please enter a number.")
```

### 7. 表格输出实现

**所需功能:**

- 列自动宽度计算
- 文本对齐 (左对齐/右对齐/居中)
- 边框绘制
- 标题行

**Python 使用 `tabulate`:**

```python
from tabulate import tabulate

def list_scripts(scripts: List[Script], plain: bool):
    if plain:
        for script in scripts:
            print(f"{script.name}.{script.extension}")
    else:
        table_data = [
            [i + 1, script.name, script.extension]
            for i, script in enumerate(scripts)
        ]

        headers = ["#", "Name", "Extension"]
        print(tabulate(table_data, headers=headers, tablefmt="grid"))
```

**输出示例:**

```
+-----+-------------+-------------+
|   # | Name        | Extension   |
+=====+=============+=============+
|   1 | backup      | sh          |
+-----+-------------+-------------+
|   2 | deploy      | py          |
+-----+-------------+-------------+
|   3 | analyze     | rb          |
+-----+-------------+-------------+
```

---

## 项目结构模板

### Python 项目结构

```
rune/
├── rune/
│   ├── __init__.py
│   ├── cli.py              # CLI 定义 (argparse/click)
│   ├── commands/
│   │   ├── __init__.py
│   │   ├── run.py          # run 命令
│   │   ├── script.py       # script 命令
│   │   └── plugin.py       # plugin 命令
│   ├── core/
│   │   ├── __init__.py
│   │   ├── executor.py     # 命令执行器
│   │   ├── plugin.py       # 插件模型和数据库
│   │   └── script.py       # 脚本加载
│   └── utils/
│       ├── __init__.py
│       ├── fs.py           # 路径工具
│       └── cli.py          # 交互式提示
├── tests/
│   ├── __init__.py
│   ├── test_plugin.py
│   ├── test_script.py
│   └── test_executor.py
├── setup.py                # 安装配置
├── requirements.txt        # 依赖列表
└── README.md
```

### Go 项目结构

```
rune/
├── cmd/
│   └── rune/
│       └── main.go         # 入口点
├── internal/
│   ├── commands/
│   │   ├── run.go
│   │   ├── script.go
│   │   └── plugin.go
│   ├── core/
│   │   ├── executor.go
│   │   ├── plugin.go
│   │   └── script.go
│   └── utils/
│       ├── fs.go
│       └── cli.go
├── pkg/                    # 公共库 (如需要)
├── tests/
├── go.mod
├── go.sum
└── README.md
```

### Node.js 项目结构

```
rune/
├── src/
│   ├── cli.js              # CLI 定义 (commander)
│   ├── commands/
│   │   ├── run.js
│   │   ├── script.js
│   │   └── plugin.js
│   ├── core/
│   │   ├── executor.js
│   │   ├── plugin.js
│   │   └── script.js
│   └── utils/
│       ├── fs.js
│       └── cli.js
├── tests/
│   ├── plugin.test.js
│   ├── script.test.js
│   └── executor.test.js
├── package.json
├── package-lock.json
└── README.md
```

---

## 常见问题和解决方案

### Q1: 如何处理脚本同名但扩展名不同?

**场景:** 存在 `backup.sh` 和 `backup.py`

**解决方案:**

- 默认行为: 要求用户指定扩展名
- 命令: `rune run backup -e sh` 或 `rune run backup -e py`
- 错误消息: "Multiple scripts named 'backup' found. Specify extension with -e"

**实现:**

```python
def find_script(scripts, name, extension=None):
    matches = [s for s in scripts if s.name == name]

    if not matches:
        raise ValueError(f"Script '{name}' not found")

    if extension:
        matches = [s for s in matches if s.extension == extension]
        if not matches:
            raise ValueError(f"Script '{name}.{extension}' not found")

    if len(matches) > 1:
        exts = [s.extension for s in matches]
        raise ValueError(
            f"Multiple scripts named '{name}' found: {', '.join(exts)}. "
            "Specify extension with -e"
        )

    return matches[0]
```

### Q2: 如何处理插件的 arg_template 中的多个 {file}?

**场景:** 某些执行器需要多次引用脚本路径

```yaml
arg_template: ["--script", "{file}", "--output", "{file}.log"]
```

**解决方案:** 替换所有出现的 `{file}`

```python
def build_command_args(plugin, script_path, user_args):
    args = []

    for template in plugin.arg_template:
        # 替换所有 {file} 占位符
        arg = template.replace('{file}', str(script_path))
        args.append(arg)

    # 追加用户参数
    args.extend(user_args)

    return args
```

### Q3: 如何处理需要特殊权限的脚本?

**场景:** 脚本需要 sudo 或管理员权限

**解决方案 1:** 用户手动在插件中配置

```yaml
name: bash-sudo
executor: sudo
arg_template: ["bash", "{file}"]
extensions: ["sh"]
```

**解决方案 2:** 脚本内部使用 `sudo`

```bash
#!/bin/bash
sudo systemctl restart nginx
```

**推荐:** 方案 2,保持工具简单

### Q4: 如何支持需要编译的脚本 (如 C/C++)?

**场景:** 脚本需要先编译再执行

**解决方案:** 创建包装脚本或使用构建工具

**选项 1: Shell 包装脚本**

```bash
# compile-and-run.sh
#!/bin/bash
gcc "$1" -o /tmp/executable
/tmp/executable "${@:2}"
```

**选项 2: 专用插件**

```yaml
name: gcc
executor: bash
arg_template:
  - "-c"
  - "gcc '{file}' -o /tmp/out && /tmp/out"
extensions: ["c"]
```

**推荐:** Rune 专注于脚本执行,编译型语言建议使用 Makefile

### Q5: 如何处理 Windows 路径 (反斜杠)?

**解决方案:** 使用标准库的路径处理

- **Python:** `pathlib.Path` (自动处理)
- **Go:** `filepath` 包 (自动处理)
- **Node.js:** `path` 模块 (自动处理)

**错误示例:**

```python
# 不要这样做
path = base + "/" + script  # 仅在 Unix 工作
```

**正确示例:**

```python
# 正确做法
from pathlib import Path
path = Path(base) / script  # 跨平台
```

---

## 性能优化建议

### 1. 脚本加载缓存

**问题:** 每次运行都扫描目录

**解决方案:** 可选的脚本索引缓存

```python
# 缓存文件: ~/.local/share/rune/script-index.json
{
    "last_updated": "2024-01-15T10:30:00Z",
    "scripts": [
        {"name": "backup", "extension": "sh", "path": "..."},
        {"name": "deploy", "extension": "py", "path": "..."}
    ]
}

# 加载逻辑
def load_scripts_cached(path):
    cache_file = get_cache_path()

    # 检查缓存是否有效
    if cache_file.exists():
        cache_mtime = cache_file.stat().st_mtime
        dir_mtime = path.stat().st_mtime

        if cache_mtime > dir_mtime:
            # 缓存有效
            return load_from_cache(cache_file)

    # 缓存无效,重新扫描
    scripts = scan_directory(path)
    save_to_cache(cache_file, scripts)
    return scripts
```

### 2. 插件数据库内存映射

**适用于:** 大量插件 (>100 个)

**解决方案:** 使用内存映射文件 (mmap)

```python
import mmap

# 避免每次运行都反序列化整个数据库
# 按需加载插件
```

**注意:** 对于 Rune 的典型使用 (< 50 插件),不需要此优化

### 3. 并行脚本列表

**场景:** 包含大量脚本的目录

**解决方案:** 使用线程/协程并行读取

**注意:** 文件系统 I/O 通常是瓶颈,并行效果有限

---

## 安全考虑

### 1. 路径遍历攻击防护

**风险:** 用户提供的路径可能包含 `../`

**防护措施:**

```python
def safe_join(base, user_path):
    """安全地拼接路径,防止遍历到父目录"""
    full_path = os.path.realpath(os.path.join(base, user_path))

    if not full_path.startswith(base):
        raise ValueError("Path traversal detected")

    return full_path
```

### 2. 命令注入防护

**风险:** 用户提供的参数可能包含 shell 特殊字符

**防护措施:**

- **不要使用 shell=True** - 直接传递参数列表
- **不要手动拼接命令字符串**

**错误示例:**

```python
# 危险!
os.system(f"{executor} {file} {user_arg}")
```

**正确示例:**

```python
# 安全
subprocess.run([executor, file, user_arg])
```

### 3. YAML 炸弹防护

**风险:** 恶意 YAML 文件可能导致资源耗尽

**防护措施:** 使用 `safe_load()` 而非 `load()`

```python
# 安全
yaml.safe_load(content)

# 危险 (允许任意 Python 对象)
yaml.load(content)
```

---

## 附录 A: 完整 CLI 帮助文本

```
Rune - Universal Script Runner

USAGE:
    rune <COMMAND> [OPTIONS]

COMMANDS:
    run, r       Run a script
    script, s    Manage scripts
    plugin, p    Manage plugins
    help         Print this message or the help of subcommands

OPTIONS:
    -h, --help       Print help
    -V, --version    Print version

---

rune run [OPTIONS] <NAME> [-- <ARGS>...]

ARGUMENTS:
    <NAME>         Script name
    [ARGS]...      Arguments to pass to the script

OPTIONS:
    -e, --extension <EXT>    Specify file extension
    -p, --plugin <PLUGIN>    Use specific plugin
    -h, --help               Print help

EXAMPLES:
    rune run backup
    rune run deploy -e py
    rune run process -p python -- input.csv output.csv

---

rune script <COMMAND>

COMMANDS:
    add       Add a script to the repository
    remove    Remove a script
    list      List all scripts
    new       Create a new script
    edit      Edit a script

---

rune plugin <COMMAND>

COMMANDS:
    add       Add a plugin
    remove    Remove a plugin
    list      List all plugins
    info      Show plugin information
    export    Export plugin to YAML
```

---

## 附录 B: 插件示例集

### Bash 插件

```yaml
name: bash
executor: bash
arg_template: ["{file}"]
extensions: [sh, bash]
description: "Bash shell script executor"
author: "Rune Team"
version: "1.0.0"
```

### Python 插件

```yaml
name: python
executor: python3
arg_template: ["-u", "{file}"]
extensions: [py, pyw]
description: "Python 3 script executor with unbuffered output"
author: "Rune Team"
version: "1.0.0"
requires:
  - pip3
```

### Node.js 插件

```yaml
name: node
executor: node
arg_template: ["{file}"]
extensions: [js, mjs]
description: "Node.js script executor"
author: "Rune Team"
version: "1.0.0"
requires:
  - npm
```

### Ruby 插件

```yaml
name: ruby
executor: ruby
arg_template: ["{file}"]
extensions: [rb]
description: "Ruby script executor"
author: "Rune Team"
version: "1.0.0"
requires:
  - gem
```

### PHP 插件

```yaml
name: php
executor: php
arg_template: ["{file}"]
extensions: [php]
description: "PHP script executor"
author: "Rune Team"
version: "1.0.0"
```

### Perl 插件

```yaml
name: perl
executor: perl
arg_template: ["{file}"]
extensions: [pl, pm]
description: "Perl script executor"
author: "Rune Team"
version: "1.0.0"
```

### Deno 插件

```yaml
name: deno
executor: deno
arg_template: ["run", "--allow-all", "{file}"]
extensions: [ts, tsx]
description: "Deno TypeScript runtime"
author: "Rune Team"
version: "1.0.0"
```

### Lua 插件

```yaml
name: lua
executor: lua
arg_template: ["{file}"]
extensions: [lua]
description: "Lua script executor"
author: "Rune Team"
version: "1.0.0"
```

---

## 附录 C: 错误消息目录

### 脚本相关错误

| 错误代码 | 消息 | 原因 |
|----------|------|------|
| E001 | Script '{name}' not found | 脚本不存在 |
| E002 | Multiple scripts named '{name}' found | 同名多扩展名 |
| E003 | Script directory does not exist | 脚本目录未创建 |
| E004 | Failed to read script directory | 权限问题 |

### 插件相关错误

| 错误代码 | 消息 | 原因 |
|----------|------|------|
| E101 | Plugin '{name}' not found | 插件不存在 |
| E102 | No plugin supports '.{ext}' extension | 没有匹配的插件 |
| E103 | Plugin '{name}' does not support '.{ext}' | 插件不支持扩展名 |
| E104 | Failed to parse plugin YAML | YAML 格式错误 |
| E105 | Plugin name is required | 缺少必需字段 |
| E106 | Executor is required | 缺少执行器 |
| E107 | At least one extension is required | 缺少扩展名 |

### 执行相关错误

| 错误代码 | 消息 | 原因 |
|----------|------|------|
| E201 | Executor '{cmd}' not found in PATH | 执行器不存在 |
| E202 | Command failed with exit code {code} | 脚本执行失败 |
| E203 | Failed to execute '{cmd}' | 系统级错误 |

### 数据库相关错误

| 错误代码 | 消息 | 原因 |
|----------|------|------|
| E301 | Failed to read plugin database | 文件损坏 |
| E302 | Failed to write plugin database | 权限或磁盘问题 |
| E303 | Database version mismatch | 需要升级 |

---

## 结语

本文档提供了使用其他编程语言重建 Rune 项目所需的全部信息:

✓ **架构设计** - 分层架构和模块划分
✓ **数据模型** - 完整的数据结构定义
✓ **核心算法** - 详细的执行流程
✓ **实现细节** - 每个模块的伪代码和示例
✓ **测试策略** - 单元测试和集成测试方案
✓ **跨平台支持** - 不同操作系统的兼容性处理
✓ **实现路线图** - 分阶段开发计划
✓ **最佳实践** - 安全性、性能、错误处理

使用本文档,你可以:

1. 在 **Python/Go/Node.js/Java** 等语言中实现等效项目
2. 理解 Rune 的设计理念和技术决策
3. 扩展功能或修改行为
4. 为原项目贡献代码 (理解架构后)

如有疑问或需要澄清,请参考源代码: `/Users/yog/CodeBase/active/rune-rs/`

**项目版本:** 0.5.0
**文档生成日期:** 2025-10-27
**作者:** Claude (基于 Rune-RS 项目分析)
