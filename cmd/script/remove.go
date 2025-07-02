package script

import (
	"context"
	"os"
	"strings"

	"github.com/inario-kitsune/rune/plugin"
	"github.com/inario-kitsune/rune/util"
	"github.com/urfave/cli/v3"
)

var ScriptRemoveCommand = &cli.Command{
	Name:    "remove",
	Aliases: []string{"rm"},
	Flags: []cli.Flag{
		&cli.StringFlag{
			Name:    "extension",
			Aliases: []string{"x"},
			Usage:   "Force script extension (e.g. py,lua)",
		},
	},
	Usage:     "Remove a script",
	ArgsUsage: "<name>",
	Action: func(ctx context.Context, c *cli.Command) error {
		name := c.Args().First()
		if name == "" {
			return cli.Exit("Missing name", 1)
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
		path, err := util.GetScriptPathFromName(name, exts)
		if err != nil {
			return err
		}
		return os.Remove(path)
	},
}
