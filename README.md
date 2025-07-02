# rune

**rune** is a universal script runner with a pluggable Lua-based execution system. It detects the script type by file extension, loads the corresponding Lua plugin, and executes the script. It also includes minimal script and plugin management capabilities.

---

## Features

* ✨ Universal script runner with language/plugin auto-detection
* 🔌 Lua-based plugin system for flexible script execution
* 🔎 Embedded plugins supported via Go `embed`, overrideable by external files
* 🛠️ Script and plugin management: list, create, run
* ⚙️ Zero-config usage with environment variable and fallback defaults

---

## Project Structure

```
rune/
├── cmd/             # CLI command implementations
├── plugin/          # Plugin handling logic
├── util/            # Helper utilities
├── scripts/         # Script repository
├── main.go
└── go.mod
```

---

## Installation

```bash
git clone https://github.com/yourname/rune.git
cd rune
go build -o rune .
./rune run hello
```

> Requires Go 1.20 or later.

---

## CLI Usage

### Script Commands

```bash
NAME:
   rune plugin - Plugin related commands

USAGE:
   rune plugin [command [command options]]

COMMANDS:
   list, ls    List available plugins
   new, n      Create a new plugin
   edit, e     Edit plugin
   remove, rm  Remove a plugin
   install, i  Install a plugin file
```

### Plugin Commands

```bash
NAME:
   rune plugin - Plugin related commands

USAGE:
   rune plugin [command [command options]]

COMMANDS:
   list, ls    List available plugins
   new, n      Create a new plugin
   edit, e     Edit plugin
   remove, rm  Remove a plugin
   install, i  Install a plugin file
```

---

## Plugin System

Plugins are Lua scripts with YAML metadata embedded in a comment block:

```lua
--[[
rune-meta
name: Python
ext: [py, py3]
]]

os.execute("python3 " .. target .. " " .. table.concat(args, " "))
```

### Plugin Requirements

* Must include `--[[ rune-meta ... ]]` at the top
* Global variables provided:

  * `target`: the script path
  * `args`: script arguments (as Lua array)

---

## Environment Variables

| Variable      | Description                                                    |
| ------------- | -------------------------------------------------------------- |
| `RUNE_PLUGIN` | Plugin directory path (default: `$XDG_DATA_HOME/rune/plugins`) |
| `RUNE_REPO`   | Script directory path (default: `$XDG_DATA_HOME/rune/scripts`) |
| `RUNE_LOG`    | Log level: `debug`, `info`, `warn`, `error`                    |
| `EDITOR`      | Editor for creating new plugins or scripts                     |

---

## Example Workflow

```bash
$ rune script new hello.py
$ rune run hello arg1 arg2
$ rune plugin list
```

---

## Developer Notes

* Plugins are loaded at runtime; external files override embedded versions.
* Logging uses [charmbracelet/log](https://github.com/charmbracelet/log) for color-coded structured output.
* Plugin files can be embedded using Go's `embed` directive for distribution.

---

## License

MIT License © 2025 Inario.
