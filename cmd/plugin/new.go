package plugin

import (
	"context"
	"fmt"
	"os"
	"strings"

	"github.com/inario-kitsune/rune/util"
	"github.com/urfave/cli/v3"
)

var PluginNewCommand = &cli.Command{
	Name:      "new",
	Aliases:   []string{"n"},
	Usage:     "Create a new plugin",
	ArgsUsage: "<name>",
	Action: func(ctx context.Context, c *cli.Command) error {
		name := c.Args().First()
		if name == "" {
			return cli.Exit("Missing name", 1)
		}
		pluginPath := util.GetPluginPath()
		path := fmt.Sprintf("%s/%s.lua", pluginPath, name)
		if _, err := os.Stat(path); err == nil {
			return cli.Exit("Plugin already exists", 1)
		}
		content := fmt.Sprintf("--[[ rune-meta\nname: %s Plugin\next: []\n]]\nprint('[plugin:%s] Running:',target)\n", strings.ToUpper(name), name)
		os.WriteFile(path, []byte(content), 0644)

		return util.OpenWithEditor(path)
	},
}
