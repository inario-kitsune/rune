package plugin

import (
	"context"
	"fmt"
	"os"

	"github.com/charmbracelet/log"
	"github.com/inario-kitsune/rune/util"
	"github.com/urfave/cli/v3"
)

var PluginRemoveCommand = &cli.Command{
	Name:      "remove",
	Aliases:   []string{"rm"},
	Usage:     "Remove a plugin",
	ArgsUsage: "<name>",
	Action: func(ctx context.Context, c *cli.Command) error {
		log.Debug("Starting plugin remove command")

		name := c.Args().First()
		log.Debug("Retrieved plugin name from arguments", "name", name)

		if name == "" {
			log.Error("Plugin name is required but not provided")
			return cli.Exit("Missing name", 1)
		}

		log.Info("Starting to remove plugin", "plugin", name)

		pluginPath := util.GetPluginPath()
		log.Debug("Retrieved plugin path", "path", pluginPath)

		path := fmt.Sprintf("%s/%s.lua", pluginPath, name)
		log.Debug("Constructed plugin file path", "file_path", path)

		log.Debug("Checking if plugin file exists", "path", path)
		if _, err := os.Stat(path); err != nil {
			if os.IsNotExist(err) {
				log.Error("Plugin file does not exist", "path", path)
				return cli.Exit("Plugin not exists", 1)
			}
			log.Error("Failed to check plugin file status", "path", path, "error", err)
			return cli.Exit(fmt.Sprintf("Error checking plugin file: %v", err), 1)
		}

		log.Info("Plugin file found, attempting to remove", "path", path)

		log.Debug("Attempting to remove plugin file", "path", path)
		if err := os.Remove(path); err != nil {
			log.Error("Failed to remove plugin file", "path", path, "error", err)
			return cli.Exit(fmt.Sprintf("Failed to remove plugin: %v", err), 1)
		}

		log.Info("Plugin removed successfully", "name", name, "path", path)
		log.Debug("Plugin remove command completed successfully")

		return nil
	},
}
