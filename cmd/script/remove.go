package script

import (
	"context"
	"os"
	"strings"

	"github.com/charmbracelet/log"
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
			log.Error("Script name is required")
			return cli.Exit("Missing name", 1)
		}

		log.Info("Starting script removal", "name", name)

		pluginPath := util.GetPluginPath()
		log.Debug("Plugin path retrieved", "path", pluginPath)

		if err := plugin.LoadPlugins(pluginPath); err != nil {
			log.Error("Failed to load plugins", "error", err, "path", pluginPath)
			return err
		}
		log.Debug("Plugins loaded successfully")

		exts := util.Keys(plugin.GetPluginList())
		log.Debug("Available extensions", "extensions", exts)

		selectedExt := c.String("extension")
		if selectedExt != "" {
			selectedExt = strings.ToLower(strings.TrimPrefix(selectedExt, "."))
			log.Debug("Extension specified", "extension", selectedExt)

			if plugin.GetPluginByExt(selectedExt) == nil {
				log.Error("No plugin registered for extension", "extension", selectedExt)
				return cli.Exit("no plugin registered for extension: "+selectedExt, 1)
			}
			exts = []string{selectedExt}
			log.Debug("Using specified extension only", "extension", selectedExt)
		}

		path, err := util.GetScriptPathFromName(name, exts)
		if err != nil {
			log.Error("Failed to get script path", "error", err, "name", name, "extensions", exts)
			return err
		}
		log.Debug("Script path resolved", "path", path)

		log.Info("Removing script file", "path", path)
		if err := os.Remove(path); err != nil {
			log.Error("Failed to remove script file", "error", err, "path", path)
			return err
		}

		log.Info("Script removed successfully", "name", name, "path", path)
		log.Debug("Script remove command completed successfully")
		return nil
	},
}
