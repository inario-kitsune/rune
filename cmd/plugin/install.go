package plugin

import (
	"context"
	"fmt"
	"os"
	"path/filepath"

	"github.com/charmbracelet/log"
	"github.com/inario-kitsune/rune/plugin"
	"github.com/inario-kitsune/rune/util"
	"github.com/urfave/cli/v3"
)

var PluginInstallCommand = &cli.Command{
	Name:      "install",
	Aliases:   []string{"i"},
	Usage:     "Install a plugin file",
	ArgsUsage: "<Lua file>",
	Action: func(ctx context.Context, c *cli.Command) error {
		log.Debug("Starting plugin install command")

		sourceFile := c.Args().First()
		log.Debug("Retrieved source file from arguments", "source_file", sourceFile)

		if sourceFile == "" {
			log.Error("Source file is required but not provided")
			return cli.Exit("Missing source file", 1)
		}

		log.Info("Attempting to install plugin", "source_file", sourceFile)

		log.Debug("Checking if source file exists", "source_file", sourceFile)
		if _, err := os.Stat(sourceFile); err != nil {
			if os.IsNotExist(err) {
				log.Error("Source file does not exist", "source_file", sourceFile)
				return cli.Exit(fmt.Sprintf("Source file %s does not exist", sourceFile), 1)
			}
			log.Error("Failed to check source file status", "source_file", sourceFile, "error", err)
			return cli.Exit(fmt.Sprintf("Error checking source file: %v", err), 1)
		}
		log.Debug("Source file exists and is accessible", "source_file", sourceFile)

		log.Debug("Starting plugin validation", "source_file", sourceFile)
		if !plugin.Validate(sourceFile) {
			log.Error("Plugin validation failed", "source_file", sourceFile)
			return fmt.Errorf("Plugin %s is invalid", sourceFile)
		}
		log.Info("Plugin validation successful", "source_file", sourceFile)

		pluginPath := util.GetPluginPath()
		log.Debug("Retrieved plugin directory path", "plugin_path", pluginPath)

		fileName := filepath.Base(sourceFile)
		targetPath := filepath.Join(pluginPath, fileName)
		log.Debug("Constructed target path", "target_path", targetPath, "file_name", fileName)

		log.Debug("Checking if target plugin already exists", "target_path", targetPath)
		if _, err := os.Stat(targetPath); err == nil {
			log.Error("Plugin already exists at target location", "target_path", targetPath)
			return cli.Exit("Plugin already exists", 1)
		} else if !os.IsNotExist(err) {
			log.Error("Failed to check target path status", "target_path", targetPath, "error", err)
			return cli.Exit(fmt.Sprintf("Error checking target path: %v", err), 1)
		}
		log.Debug("Target path is available for installation", "target_path", targetPath)

		log.Debug("Ensuring plugin directory exists", "plugin_path", pluginPath)
		if err := os.MkdirAll(pluginPath, 0755); err != nil {
			log.Error("Failed to create plugin directory", "plugin_path", pluginPath, "error", err)
			return cli.Exit(fmt.Sprintf("Failed to create plugin directory: %v", err), 1)
		}
		log.Debug("Plugin directory is ready", "plugin_path", pluginPath)

		log.Info("Installing plugin", "source_file", sourceFile, "target_path", targetPath)
		log.Debug("Attempting to move file", "from", sourceFile, "to", targetPath)

		if err := os.Rename(sourceFile, targetPath); err != nil {
			log.Error("Failed to install plugin", "source_file", sourceFile, "target_path", targetPath, "error", err)
			return cli.Exit(fmt.Sprintf("Failed to install plugin: %v", err), 1)
		}
		log.Info("Successfully install plugin file",
			"filename", filepath.Base(sourceFile),
			"from", sourceFile,
			"to", targetPath)
		log.Debug("Plugin install command completed successfully")

		return nil
	},
}
