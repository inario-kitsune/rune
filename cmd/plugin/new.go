package plugin

import (
	"context"
	"fmt"
	"os"
	"strings"

	"github.com/charmbracelet/log"
	"github.com/inario-kitsune/rune/util"
	"github.com/urfave/cli/v3"
)

var PluginNewCommand = &cli.Command{
	Name:      "new",
	Aliases:   []string{"n"},
	Usage:     "Create a new plugin",
	ArgsUsage: "<name>",
	Action: func(ctx context.Context, c *cli.Command) error {
		log.Debug("Starting plugin new command")

		name := c.Args().First()
		log.Debug("Retrieved plugin name from arguments", "name", name)

		if name == "" {
			log.Error("Plugin name is required but not provided")
			return cli.Exit("Missing name", 1)
		}

		log.Info("Creating new plugin", "plugin", name)

		pluginPath := util.GetPluginPath()
		log.Debug("Retrieved plugin path", "path", pluginPath)

		path := fmt.Sprintf("%s/%s.lua", pluginPath, name)
		log.Debug("Constructed plugin file path", "file_path", path)

		log.Debug("Checking if plugin file already exists", "path", path)
		if _, err := os.Stat(path); err == nil {
			log.Error("Plugin file already exists", "path", path)
			return cli.Exit("Plugin already exists", 1)
		} else if !os.IsNotExist(err) {
			log.Error("Failed to check plugin file status", "path", path, "error", err)
			return cli.Exit(fmt.Sprintf("Error checking plugin file: %v", err), 1)
		}

		log.Debug("Plugin file does not exist, proceeding with creation")

		upperName := strings.ToUpper(name)
		content := fmt.Sprintf("--[[ rune-meta\nname: %s Plugin\next: []\n]]\nprint('[plugin:%s] Running:',target)\n", upperName, name)
		log.Debug("Generated plugin content", "plugin", name, "content_length", len(content))

		log.Debug("Writing plugin content to file", "path", path, "content_size", len(content))
		if err := os.WriteFile(path, []byte(content), 0644); err != nil {
			log.Error("Failed to write plugin file", "path", path, "error", err)
			return cli.Exit(fmt.Sprintf("Failed to create plugin file: %v", err), 1)
		}

		log.Info("Successfully created plugin file", "plugin", name, "path", path)

		log.Debug("Attempting to open new plugin file with editor", "path", path)
		if err := util.OpenWithEditor(path); err != nil {
			log.Error("Failed to open plugin file with editor", "path", path, "error", err)
			log.Warn("Plugin file was created successfully but failed to open editor", "plugin", name, "path", path)
			return cli.Exit(fmt.Sprintf("Plugin created but failed to open editor: %v", err), 1)
		}

		log.Debug("Plugin new command completed successfully")

		return nil
	},
}
