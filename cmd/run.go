package cmd

import (
	"context"
	"path/filepath"
	"strings"

	"github.com/charmbracelet/log"
	"github.com/inario-kitsune/rune/plugin"
	"github.com/inario-kitsune/rune/util"
	"github.com/urfave/cli/v3"
)

var RunCommand = &cli.Command{
	Name:      "run",
	Aliases:   []string{"r"},
	Usage:     "Run a script using appropriate plugin",
	ArgsUsage: "<script> [args...]",
	Flags: []cli.Flag{
		&cli.StringFlag{
			Name:    "extension",
			Aliases: []string{"x"},
			Usage:   "Force script extension (e.g. py,lua)",
		},
	},
	Action: func(ctx context.Context, c *cli.Command) error {
		log.Info("Starting script execution")

		scriptName := c.Args().First()
		if scriptName == "" {
			log.Error("Script name is required")
			return cli.Exit("missing script name", 1)
		}
		log.Debug("Script name", "name", scriptName)

		pluginPath := util.GetPluginPath()
		log.Debug("Loading plugins", "path", pluginPath)

		if err := plugin.LoadPlugins(pluginPath); err != nil {
			log.Error("Failed to load plugins", "error", err, "path", pluginPath)
			return err
		}
		log.Info("Plugins loaded successfully")

		exts := util.Keys(plugin.GetPluginList())
		log.Debug("Available extensions", "extensions", exts)

		selectedExt := c.String("extension")
		if selectedExt != "" {
			selectedExt = strings.ToLower(strings.TrimPrefix(selectedExt, "."))
			log.Debug("Extension forced", "extension", selectedExt)

			if plugin.GetPluginByExt(selectedExt) == nil {
				log.Error("No plugin found for extension", "extension", selectedExt)
				return cli.Exit("no plugin registered for extension: "+selectedExt, 1)
			}
			exts = []string{selectedExt}
			log.Debug("Using forced extension", "extension", selectedExt)
		}

		log.Debug("Searching for script file", "script", scriptName, "extensions", exts)
		scriptFile, err := util.GetScriptPathFromName(scriptName, exts)
		if err != nil {
			log.Error("Failed to find script file", "error", err, "script", scriptName)
			return err
		}
		log.Info("Script file found", "path", scriptFile)

		ext := strings.ToLower(strings.TrimPrefix(filepath.Ext(scriptFile), "."))
		log.Debug("Detected extension", "extension", ext)

		plugin := plugin.GetPluginByExt(ext)
		if plugin == nil {
			log.Error("No plugin available for extension", "extension", ext)
			return cli.Exit("no plugin for extension", 1)
		}
		log.Info("Plugin selected", "extension", ext)

		args := c.Args().Slice()[1:]
		log.Debug("Script arguments", "args", args)

		log.Info("Executing script", "file", scriptFile, "args", args)
		if err := plugin.Run(scriptFile, args); err != nil {
			log.Error("Script execution failed", "error", err, "file", scriptFile)
			return err
		}

		log.Info("Script executed successfully", "file", scriptFile)
		log.Debug("Run command completed successfully")
		return nil
	},
}
