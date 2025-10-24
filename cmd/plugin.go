package cmd

import (
	"github.com/inario-kitsune/rune/cmd/plugin"
	"github.com/urfave/cli/v3"
)

var PluginCommand = &cli.Command{
	Name:    "plugin",
	Aliases: []string{"p"},
	Usage:   "Plugin related commands",
	Commands: []*cli.Command{
		plugin.PluginListCommand,
		plugin.PluginNewCommand,
		plugin.PluginEditCommand,
		plugin.PluginRemoveCommand,
		plugin.PluginInstallCommand,
	},
}
