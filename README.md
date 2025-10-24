# Rune - 通用脚本运行器

<div align="center">

![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)
![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)
![Tests](https://img.shields.io/badge/tests-36%20passed-brightgreen.svg)

[English](./README_EN.md) | 简体中文

一个强大的、语言无关的脚本管理和执行工具

</div>

## ✨ 特性

- 🚀 **集中式管理** - 将所有脚本存储在统一的仓库中，随处可用
- 🔌 **插件系统** - 通过插件支持任何脚本语言（Python、Shell、Ruby、R 等）
- 🎯 **智能执行** - 根据文件扩展名自动选择合适的执行器
- 🛠 **灵活配置** - YAML 格式的插件定义，支持自定义参数模板
- ⚡ **高性能** - 使用 Rust 编写，快速且可靠
- 🧪 **完整测试** - 36 个单元测试，100% 通过率

## 📦 安装

### 从源码构建

```bash
git clone https://github.com/yourusername/rune-rs.git
cd rune-rs
cargo build --release
sudo cp target/release/rune /usr/local/bin/
```

### 使用 Cargo

```bash
cargo install --path .
```

## 🚀 快速开始

### 1. 添加插件

首先创建一个插件定义文件 `python.yaml`：

```yaml
name: python
executor: python3
arg_template:
  - "{file}"
extensions:
  - py
description: Python 3 解释器
author: Your Name
version: 1.0.0
```

然后添加插件：

```bash
rune plugin add python.yaml
```

### 2. 添加脚本

将脚本添加到 Rune 仓库：

```bash
rune script add /path/to/your/script.py
```

### 3. 运行脚本

```bash
# 运行脚本（自动检测插件）
rune run script-name

# 指定插件运行
rune run script-name -p python

# 传递参数给脚本
rune run script-name -- arg1 arg2 arg3
```

## 📖 详细用法

### 脚本管理

```bash
# 添加脚本
rune script add /path/to/backup.sh
rune script add ~/scripts/deploy.py

# 列出所有脚本
rune script list

# 列出脚本（简洁模式）
rune script list --plain

# 创建新脚本
rune script new my-script.sh

# 编辑脚本
rune script edit backup

# 删除脚本
rune script remove backup
rune script remove backup -x sh  # 指定扩展名
rune script remove backup -y     # 跳过确认
```

### 插件管理

```bash
# 添加插件
rune plugin add python.yaml

# 强制覆盖已存在的插件
rune plugin add python.yaml --force

# 列出所有插件
rune plugin list

# 查看插件详情
rune plugin info python

# 导出插件定义
rune plugin export python
rune plugin export python -o python-backup.yaml

# 删除插件
rune plugin remove python
rune plugin remove python -y  # 跳过确认
```

### 运行脚本

```bash
# 基本运行
rune run backup

# 指定扩展名（当有重名脚本时）
rune run convert -x py

# 指定使用的插件
rune run script -p python

# 传递参数
rune run process-data -- input.csv output.csv --verbose

# 命令别名
rune r backup    # 运行
rune s list      # 脚本列表
rune p list      # 插件列表
```

## 🔌 插件系统

### 插件定义格式

```yaml
# 必填字段
name: plugin-name              # 插件唯一标识
executor: command              # 执行器命令（如 python3, bash）
extensions:                    # 支持的文件扩展名列表
  - py
  - pyw

# 可选字段
arg_template:                  # 参数模板（默认为 ["{file}"]）
  - "-u"                       # 无缓冲模式
  - "{file}"                   # {file} 会被替换为脚本路径
description: Python 3 解释器   # 插件描述
author: Your Name              # 作者
version: 1.0.0                 # 版本
requires:                      # 依赖的其他命令
  - pip3
  - virtualenv
```

### 内置插件示例

#### Python 插件

```yaml
name: python
executor: python3
arg_template:
  - "-u"
  - "{file}"
extensions:
  - py
description: Python 3 interpreter with unbuffered output
```

#### Shell 插件

```yaml
name: bash
executor: bash
arg_template:
  - "{file}"
extensions:
  - sh
  - bash
description: Bash shell interpreter
```

#### Node.js 插件

```yaml
name: node
executor: node
arg_template:
  - "{file}"
extensions:
  - js
  - mjs
description: Node.js JavaScript runtime
```

#### Ruby 插件

```yaml
name: ruby
executor: ruby
arg_template:
  - "{file}"
extensions:
  - rb
description: Ruby interpreter
```

## 📁 目录结构

Rune 使用以下目录存储数据：

### Linux / macOS

```
~/.local/share/rune/
├── scripts/          # 脚本存储目录
│   ├── backup.sh
│   ├── deploy.py
│   └── process.rb
└── plugin/
    └── plugin.dat    # 插件数据库（二进制）
```

### Windows

```
%APPDATA%\rune\
├── scripts\
└── plugin\
    └── plugin.dat
```

### 环境变量覆盖

```bash
# 自定义脚本目录
export RUNE_REPO=/custom/path/to/scripts

# 自定义插件目录
export RUNE_PLUGIN=/custom/path/to/plugin
```

## 🎨 使用场景

### 1. 个人脚本库

将所有常用脚本集中管理：

```bash
rune script add ~/backup-database.sh
rune script add ~/deploy-website.py
rune script add ~/clean-logs.sh

# 随处可用
rune run backup-database
rune run deploy-website
```

### 2. 多语言项目

在一个项目中使用多种脚本语言：

```bash
rune run preprocess-data    # Python 脚本
rune run build-assets       # Shell 脚本
rune run analyze-results    # R 脚本
```

### 3. 团队协作

导出和分享插件配置：

```bash
# 导出插件
rune plugin export python -o python.yaml

# 团队成员导入
rune plugin add python.yaml
```

## 🧪 测试

Rune 拥有完整的测试套件：

```bash
# 运行所有测试
cargo test

# 运行单元测试
cargo test --lib

# 运行特定测试
cargo test test_load_scripts

# 查看测试输出
cargo test -- --nocapture
```

测试统计：
- **总测试数**: 36
- **通过率**: 100%
- **覆盖模块**: core/script, core/plugin, core/executor

详见 [TESTING.md](./TESTING.md) 了解测试策略。

## ⚙️ 配置

### Shell 自动补全

Rune 支持 shell 自动补全（通过 `rune.yaml` 配置）。

## 📄 许可证

本项目采用 Apache License 2.0 许可证 - 详见 [LICENSE](LICENSE) 文件

```
Copyright 2024 Yoikitsune

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

## 🙏 致谢

- [clap](https://github.com/clap-rs/clap) - 命令行参数解析
- [serde](https://github.com/serde-rs/serde) - 序列化/反序列化
- [anyhow](https://github.com/dtolnay/anyhow) - 错误处理

## 📮 联系方式

- 作者: Yoikitsune
- 项目链接: [https://github.com/yourusername/rune-rs](https://github.com/yourusername/rune-rs)

## 🗺 路线图

### v0.6.0 (计划中)
- [ ] 脚本搜索功能
- [ ] 环境变量注入
- [ ] 脚本模板系统

### v0.7.0 (计划中)
- [ ] 远程脚本仓库支持
- [ ] 脚本执行历史
- [ ] 干运行模式

### v1.0.0 (长期目标)
- [ ] 插件商店
- [ ] Web UI
- [ ] 脚本依赖管理

---

<div align="center">

**如果这个项目对你有帮助，请给一个 ⭐️！**

Made with ❤️ by Yoikitsune

</div>
