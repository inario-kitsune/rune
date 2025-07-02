package cmd

import (
	"context"
	"path/filepath"
	"strings"

	"github.com/inario-kitsune/rune/plugin"
	"github.com/inario-kitsune/rune/util"
	"github.com/urfave/cli/v3"
)

var RunCommand = &cli.Command{
	Name:      "run",
	Aliases:   []string{"r"},
	Usage:     "Run a script using appropriate plugin",
	ArgsUsage: "<script> [args...]",
	Flags: []cli.Flag{
		&cli.StringFlag{
			Name:    "extension",
			Aliases: []string{"x"},
			Usage:   "Force script extension (e.g. py,lua)",
		},
	},
	Action: func(ctx context.Context, c *cli.Command) error {
		scriptName := c.Args().First()
		if scriptName == "" {
			return cli.Exit("missing script name", 1)
		}
		pluginPath := util.GetPluginPath()
		if err := plugin.LoadPlugins(pluginPath); err != nil {
			return err
		}
		exts := util.Keys(plugin.GetPluginList())
		selectedExt := c.String("extension")
		if selectedExt != "" {
			selectedExt = strings.ToLower(strings.TrimPrefix(selectedExt, "."))
			if plugin.GetPluginByExt(selectedExt) == nil {
				return cli.Exit("no plugin registered for extension: "+selectedExt, 1)
			}
			exts = []string{selectedExt}
		}
		scriptFile, err := util.GetScriptPathFromName(scriptName, exts)
		if err != nil {
			return err
		}
		ext := strings.ToLower(strings.TrimPrefix(filepath.Ext(scriptFile), "."))
		plugin := plugin.GetPluginByExt(ext)
		if plugin == nil {
			return cli.Exit("no plugin for extension", 1)
		}
		args := c.Args().Slice()[1:]
		if err := plugin.Execute(scriptFile, args); err != nil {
			return err
		}
		return nil
	},
}
