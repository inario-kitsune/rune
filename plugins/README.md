# Rune Plugin Templates

这个目录包含了各种常用编程语言和脚本引擎的 Rune 插件模板。插件定义了如何执行特定类型的脚本文件。

## 📦 格式说明

Rune 支持两种插件配置格式：

- **TOML** (推荐) - 简洁、易读、类型安全
- **YAML** - 传统格式，向下兼容

所有模板都优先提供 **TOML 格式**，部分常用插件在 `yaml-examples/` 目录中提供 YAML 版本作为参考。

## 🚀 快速开始

### 安装插件

```bash
# 使用 TOML 格式（推荐）
rune plugin add plugins/python-uv.toml

# 或使用 YAML 格式
rune plugin add plugins/yaml-examples/python-uv.yaml

# 查看已安装的插件
rune plugin list

# 查看插件详细信息
rune plugin info python-uv
```

### 使用插件运行脚本

```bash
# 自动选择插件（根据文件扩展名）
rune run my-script.py

# 指定插件
rune run my-script.py --plugin python-uv
```

## 📚 可用插件模板

### Python 生态

| 插件名 | 文件 | 扩展名 | 说明 |
|--------|------|--------|------|
| `python-uv` | `python-uv.toml` | `.py` | 使用 uv 运行 Python（超快速包管理器） |
| `python` | `python.toml` | `.py` | 标准 Python 3 解释器 |

### Shell 脚本

| 插件名 | 文件 | 扩展名 | 说明 |
|--------|------|--------|------|
| `bash` | `bash.toml` | `.sh`, `.bash` | Bash shell 脚本 |
| `zsh` | `zsh.toml` | `.zsh` | Z Shell 脚本 |
| `nushell` | `nushell.toml` | `.nu` | Nushell - 现代结构化数据 shell |

### JavaScript/TypeScript

| 插件名 | 文件 | 扩展名 | 说明 |
|--------|------|--------|------|
| `node` | `node.toml` | `.js`, `.mjs` | Node.js JavaScript 运行时 |
| `deno` | `deno.toml` | `.ts`, `.js` | Deno - 现代 TS/JS 运行时 |

### 其他语言

| 插件名 | 文件 | 扩展名 | 说明 |
|--------|------|--------|------|
| `ruby` | `ruby.toml` | `.rb` | Ruby 解释器 |
| `perl` | `perl.toml` | `.pl`, `.pm` | Perl 解释器 |
| `lua` | `lua.toml` | `.lua` | Lua 轻量级脚本语言 |
| `php` | `php.toml` | `.php` | PHP 服务器端脚本 |

## 🔧 插件结构说明

### TOML 格式示例

```toml
# 插件名称（唯一标识）
name = "python-uv"

# 执行器命令
executor = "uv"

# 参数模板，{file} 会被替换为脚本路径
arg_template = ["run", "{file}"]

# 支持的文件扩展名
extensions = ["py"]

# 描述信息
description = "Python script executor using uv"

# 作者
author = "Rune Plugin Templates"

# 版本号
version = "1.0.0"

# 依赖的命令（用于验证）
requires = ["uv"]
```

### YAML 格式示例

```yaml
name: python-uv
executor: uv
arg_template:
  - run
  - "{file}"
extensions:
  - py
description: Python script executor using uv
author: Rune Plugin Templates
version: 1.0.0
requires:
  - uv
```

## 📝 必填字段

- `name` - 插件名称（唯一标识）
- `executor` - 执行器命令（如 `python3`, `bash`, `node`）
- `extensions` - 支持的文件扩展名列表（至少一个）

## 🎯 可选字段

- `arg_template` - 参数模板（默认值：`["{file}"]`）
- `description` - 插件描述
- `author` - 作者信息
- `version` - 版本号
- `requires` - 依赖的命令列表

## 💡 高级用法

### 自定义参数模板

某些工具需要特定的参数格式：

```toml
# Deno 需要权限标志
name = "deno"
executor = "deno"
arg_template = ["run", "--allow-all", "{file}"]
extensions = ["ts", "js"]
```

```toml
# uv 使用 'run' 子命令
name = "python-uv"
executor = "uv"
arg_template = ["run", "{file}"]
extensions = ["py"]
```

### 多扩展名支持

一个插件可以支持多个文件扩展名：

```toml
name = "bash"
executor = "bash"
extensions = ["sh", "bash"]  # 支持 .sh 和 .bash
```

```toml
name = "node"
executor = "node"
extensions = ["js", "mjs"]  # 支持 .js 和 .mjs
```

## 🔍 插件管理命令

```bash
# 添加插件
rune plugin add <path>

# 强制覆盖已存在的插件
rune plugin add <path> --force

# 删除插件
rune plugin remove <name>

# 列出所有插件
rune plugin list

# 查看插件详细信息
rune plugin info <name>

# 导出插件（YAML 格式）
rune plugin export <name> -o output.yaml

# 导出插件（TOML 格式）
rune plugin export <name> -o output.toml

# 显式指定导出格式
rune plugin export <name> -f toml -o output.toml
```

## 🛠️ 创建自定义插件

### 步骤 1: 创建插件文件

创建一个 `.toml` 或 `.yaml` 文件：

```toml
name = "my-custom-lang"
executor = "my-interpreter"
arg_template = ["{file}"]
extensions = ["mcl"]
description = "My custom language interpreter"
version = "1.0.0"
requires = ["my-interpreter"]
```

### 步骤 2: 测试执行器

确保执行器在系统 PATH 中可用：

```bash
which my-interpreter
```

### 步骤 3: 添加插件

```bash
rune plugin add my-custom-lang.toml
```

### 步骤 4: 验证

```bash
# 查看插件信息
rune plugin info my-custom-lang

# 测试运行
rune run test-script.mcl
```

## 📖 安装依赖

各插件所需工具的安装方法已在对应的 `.toml` 文件注释中提供。常见安装方式：

### macOS (Homebrew)
```bash
brew install python3 node ruby lua php
brew install uv nushell deno
```

### Ubuntu/Debian
```bash
sudo apt install python3 nodejs ruby lua5.4 php-cli perl
```

### 使用 Cargo (Rust 工具)
```bash
cargo install nu deno
```

### 使用专用安装脚本
```bash
# uv
curl -LsSf https://astral.sh/uv/install.sh | sh

# Deno
curl -fsSL https://deno.land/install.sh | sh
```

## 🤝 贡献

欢迎贡献更多语言的插件模板！创建 Pull Request 时请：

1. 优先提供 TOML 格式
2. 包含详细的注释和安装说明
3. 添加到本 README 的插件列表中
4. 确保 `version` 字段存在并合理

## 📄 许可证

这些插件模板与 Rune 项目使用相同的许可证。
