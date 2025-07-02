package plugin

import (
	"context"
	"fmt"
	"os"

	"github.com/charmbracelet/log"
	"github.com/inario-kitsune/rune/util"
	"github.com/urfave/cli/v3"
)

var PluginEditCommand = &cli.Command{
	Name:      "edit",
	Aliases:   []string{"e"},
	Usage:     "Edit plugin",
	ArgsUsage: "<name>",
	Action: func(ctx context.Context, c *cli.Command) error {
		name := c.Args().First()
		log.Info("Editing %s", name)
		if name == "" {
			return cli.Exit("Missing name", 1)
		}
		pluginPath := util.GetPluginPath()
		path := fmt.Sprintf("%s/%s.lua", pluginPath, name)
		if _, err := os.Stat(path); err != nil {
			return cli.Exit("Plugin not exist", 1)
		}
		log.Info("Editing %s", path)

		return util.OpenWithEditor(path)
	},
}
