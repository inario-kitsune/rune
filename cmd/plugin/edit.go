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
		log.Debug("Starting plugin edit command")
		name := c.Args().First()
		log.Debug("Retrieved plugin name from arguments", "name", name)
		if name == "" {
			log.Error("Plugin name is required but not provided")
			return cli.Exit("Missing name", 1)
		}
		log.Info("Starting to edit plugin", "plugin", name)

		pluginPath := util.GetPluginPath()
		log.Debug("Retrieved plugin path", "path", pluginPath)

		path := fmt.Sprintf("%s/%s.lua", pluginPath, name)
		log.Debug("Constructed plugin file path", "file_path", path)

		log.Debug("Checking if plugin file exists", "path", path)
		if _, err := os.Stat(path); err != nil {
			if os.IsNotExist(err) {
				log.Error("Plugin file does not exist", "path", path)
				return cli.Exit("Plugin not exist", 1)
			}
			log.Error("Failed to check plugin file status", "path", path, "error", err)
			return cli.Exit(fmt.Sprintf("Error checking plugin file: %v", err), 1)
		}
		log.Info("Plugin file found, opening with editor", "path", path)
		log.Debug("Attempting to open file with editor", "path", path)
		if err := util.OpenWithEditor(path); err != nil {
			log.Error("Failed to open plugin file with editor", "path", path, "error", err)
			return cli.Exit(fmt.Sprintf("Failed to open editor: %v", err), 1)
		}
		log.Debug("Plugin edit command completed successfully")

		return nil
	},
}
