package plugin

import (
	"context"
	"fmt"
	"path/filepath"
	"strings"

	"github.com/inario-kitsune/rune/plugin"
	"github.com/inario-kitsune/rune/util"
	"github.com/urfave/cli/v3"
)

var PluginListCommand = &cli.Command{
	Name:    "list",
	Aliases: []string{"ls"},
	Usage:   "List available plugins",
	Flags: []cli.Flag{
		&cli.BoolFlag{
			Name:    "plain",
			Aliases: []string{"p"},
			Usage:   "List one file per line (no formatting)",
		},
	},
	Action: func(ctx context.Context, c *cli.Command) error {
		pluginPath := util.GetPluginPath()
		err := plugin.LoadPlugins(pluginPath)
		if err != nil {
			return err
		}
		plugins := plugin.GetPluginList()

		if c.Bool("plain") {
			nameSet := make(map[string]struct{}, len(plugins))
			for _, plugin := range plugins {
				name := strings.TrimSuffix(filepath.Base(plugin.Path), ".lua")
				nameSet[name] = struct{}{}
			}
			for name := range nameSet {
				fmt.Printf("%s\n", name)
			}
		} else {
			header := []string{"Extension", "Name", "File"}
			var rows [][]string
			for ext, plugin := range plugins {
				source := "builtin:"
				if !strings.HasPrefix(plugin.Path, "builtin:") {
					source = ""
				}
				rows = append(rows, []string{
					ext,
					plugin.Name,
					source + filepath.Base(plugin.Path),
				})
			}
			util.PrintTable(header, rows)
		}
		return nil
	},
}
