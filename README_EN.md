# Rune - Universal Script Runner

<div align="center">

![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)
![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)
![Tests](https://img.shields.io/badge/tests-36%20passed-brightgreen.svg)

English | [简体中文](./README.md)

A powerful, language-agnostic script management and execution tool

</div>

## ✨ Features

- 🚀 **Centralized Management** - Store all scripts in a unified repository, accessible anywhere
- 🔌 **Plugin System** - Support any scripting language through plugins (Python, Shell, Ruby, R, etc.)
- 🎯 **Smart Execution** - Automatically select appropriate executor based on file extension
- 🛠 **Flexible Configuration** - YAML-based plugin definitions with custom argument templates
- ⚡ **High Performance** - Written in Rust for speed and reliability
- 🧪 **Fully Tested** - 36 unit tests with 100% pass rate

## 📦 Installation

### Build from Source

```bash
git clone https://github.com/yourusername/rune-rs.git
cd rune-rs
cargo build --release
sudo cp target/release/rune /usr/local/bin/
```

### Using Cargo

```bash
cargo install --path .
```

## 🚀 Quick Start

### 1. Add a Plugin

First, create a plugin definition file `python.yaml`:

```yaml
name: python
executor: python3
arg_template:
  - "{file}"
extensions:
  - py
description: Python 3 interpreter
author: Your Name
version: 1.0.0
```

Then add the plugin:

```bash
rune plugin add python.yaml
```

### 2. Add a Script

Add a script to the Rune repository:

```bash
rune script add /path/to/your/script.py
```

### 3. Run the Script

```bash
# Run script (auto-detect plugin)
rune run script-name

# Specify plugin
rune run script-name -p python

# Pass arguments to script
rune run script-name -- arg1 arg2 arg3
```

## 📖 Detailed Usage

### Script Management

```bash
# Add scripts
rune script add /path/to/backup.sh
rune script add ~/scripts/deploy.py

# List all scripts
rune script list

# List scripts (plain format)
rune script list --plain

# Create new script
rune script new my-script.sh

# Edit script
rune script edit backup

# Remove script
rune script remove backup
rune script remove backup -x sh  # Specify extension
rune script remove backup -y     # Skip confirmation
```

### Plugin Management

```bash
# Add plugin
rune plugin add python.yaml

# Force overwrite existing plugin
rune plugin add python.yaml --force

# List all plugins
rune plugin list

# View plugin details
rune plugin info python

# Export plugin definition
rune plugin export python
rune plugin export python -o python-backup.yaml

# Remove plugin
rune plugin remove python
rune plugin remove python -y  # Skip confirmation
```

### Running Scripts

```bash
# Basic run
rune run backup

# Specify extension (when duplicate names exist)
rune run convert -x py

# Specify plugin to use
rune run script -p python

# Pass arguments
rune run process-data -- input.csv output.csv --verbose

# Command aliases
rune r backup    # Run
rune s list      # Script list
rune p list      # Plugin list
```

## 🔌 Plugin System

### Plugin Definition Format

```yaml
# Required fields
name: plugin-name              # Unique plugin identifier
executor: command              # Executor command (e.g., python3, bash)
extensions:                    # Supported file extensions
  - py
  - pyw

# Optional fields
arg_template:                  # Argument template (defaults to ["{file}"])
  - "-u"                       # Unbuffered mode
  - "{file}"                   # {file} replaced with script path
description: Python 3 interpreter  # Plugin description
author: Your Name              # Author
version: 1.0.0                 # Version
requires:                      # Required dependencies
  - pip3
  - virtualenv
```

### Built-in Plugin Examples

#### Python Plugin

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

#### Shell Plugin

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

#### Node.js Plugin

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

#### Ruby Plugin

```yaml
name: ruby
executor: ruby
arg_template:
  - "{file}"
extensions:
  - rb
description: Ruby interpreter
```

## 📁 Directory Structure

Rune stores data in the following directories:

### Linux / macOS

```
~/.local/share/rune/
├── scripts/          # Script storage
│   ├── backup.sh
│   ├── deploy.py
│   └── process.rb
└── plugin/
    └── plugin.dat    # Plugin database (binary)
```

### Windows

```
%APPDATA%\rune\
├── scripts\
└── plugin\
    └── plugin.dat
```

### Environment Variable Overrides

```bash
# Custom script directory
export RUNE_REPO=/custom/path/to/scripts

# Custom plugin directory
export RUNE_PLUGIN=/custom/path/to/plugin
```

## 🎨 Use Cases

### 1. Personal Script Library

Centrally manage all frequently-used scripts:

```bash
rune script add ~/backup-database.sh
rune script add ~/deploy-website.py
rune script add ~/clean-logs.sh

# Access anywhere
rune run backup-database
rune run deploy-website
```

### 2. Multi-Language Projects

Use multiple scripting languages in one project:

```bash
rune run preprocess-data    # Python script
rune run build-assets       # Shell script
rune run analyze-results    # R script
```

### 3. Team Collaboration

Export and share plugin configurations:

```bash
# Export plugin
rune plugin export python -o python.yaml

# Team members import
rune plugin add python.yaml
```

## 🧪 Testing

Rune has a comprehensive test suite:

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run specific test
cargo test test_load_scripts

# Show test output
cargo test -- --nocapture
```

Test Statistics:
- **Total Tests**: 36
- **Pass Rate**: 100%
- **Coverage**: core/script, core/plugin, core/executor

See [TESTING.md](./TESTING.md) for testing strategy.

## ⚙️ Configuration

### Shell Completion

Rune supports shell completion (configured via `rune.yaml`).

## 📄 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details

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

## 🙏 Acknowledgments

- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing
- [serde](https://github.com/serde-rs/serde) - Serialization/deserialization
- [anyhow](https://github.com/dtolnay/anyhow) - Error handling

## 📮 Contact

- Author: Yoikitsune
- Project Link: [https://github.com/yourusername/rune-rs](https://github.com/yourusername/rune-rs)

## 🗺 Roadmap

### v0.6.0 (Planned)
- [ ] Script search functionality
- [ ] Environment variable injection
- [ ] Script template system

### v0.7.0 (Planned)
- [ ] Remote script repository support
- [ ] Script execution history
- [ ] Dry-run mode

### v1.0.0 (Long-term)
- [ ] Plugin marketplace
- [ ] Web UI
- [ ] Script dependency management

---

<div align="center">

**If this project helps you, please give it a ⭐️!**

Made with ❤️ by Yoikitsune

</div>
