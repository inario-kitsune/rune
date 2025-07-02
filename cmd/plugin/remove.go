package plugin

import (
	"context"
	"fmt"
	"os"

	"github.com/inario-kitsune/rune/util"
	"github.com/urfave/cli/v3"
)

var PluginRemoveCommand = &cli.Command{
	Name:      "remove",
	Aliases:   []string{"rm"},
	Usage:     "Remove a plugin",
	ArgsUsage: "<name>",
	Action: func(ctx context.Context, c *cli.Command) error {
		name := c.Args().First()
		if name == "" {
			return cli.Exit("Missing name", 1)
		}
		pluginPath := util.GetPluginPath()
		path := fmt.Sprintf("%s/%s.lua", pluginPath, name)
		if _, err := os.Stat(path); err != nil {
			return cli.Exit("Plugin not exists", 1)
		}
		return os.Remove(path)
	},
}
