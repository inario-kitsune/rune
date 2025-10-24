package cmd

import (
	"github.com/inario-kitsune/rune/cmd/script"
	"github.com/urfave/cli/v3"
)

var ScriptCommand = &cli.Command{
	Name:    "script",
	Aliases: []string{"s"},
	Usage:   "Plugin related commands",
	Commands: []*cli.Command{
		script.ScriptListCommand,
		script.ScriptEditCommand,
		script.ScriptNewCommand,
		script.ScriptRemoveCommand,
		script.ScriptImportCommand,
	},
}
