package script

import (
	"context"
	"strings"

	"github.com/charmbracelet/log"
	"github.com/inario-kitsune/rune/plugin"
	"github.com/inario-kitsune/rune/util"
	"github.com/urfave/cli/v3"
)

var ScriptEditCommand = &cli.Command{
	Name:    "edit",
	Aliases: []string{"e"},
	Flags: []cli.Flag{
		&cli.StringFlag{
			Name:    "extension",
			Aliases: []string{"x"},
			Usage:   "Force script extension (e.g. py,lua)",
		},
	},
	Usage:     "Edit script",
	ArgsUsage: "<script-name>",
	Action: func(ctx context.Context, c *cli.Command) error {
		log.Debug("Starting script edit command")

		name := c.Args().First()
		log.Debug("Retrieved script name from arguments", "name", name)

		if name == "" {
			log.Error("Script name is required but not provided")
			return cli.Exit("Missing name", 1)
		}

		log.Info("Starting to edit script", "script", name)

		pluginPath := util.GetPluginPath()
		log.Debug("Retrieved plugin path", "path", pluginPath)

		log.Debug("Loading plugins from path", "path", pluginPath)
		if err := plugin.LoadPlugins(pluginPath); err != nil {
			log.Error("Failed to load plugins", "path", pluginPath, "error", err)
			return err
		}
		log.Debug("Successfully loaded plugins")

		exts := util.Keys(plugin.GetPluginList())
		log.Debug("Retrieved available plugin extensions", "extensions", exts)

		selectedExt := c.String("extension")
		log.Debug("Retrieved extension flag", "extension", selectedExt)

		if selectedExt != "" {
			log.Info("Using specified extension", "extension", selectedExt)

			originalExt := selectedExt
			selectedExt = strings.ToLower(strings.TrimPrefix(selectedExt, "."))
			log.Debug("Normalized extension", "original", originalExt, "normalized", selectedExt)

			log.Debug("Checking if plugin exists for extension", "extension", selectedExt)
			if plugin.GetPluginByExt(selectedExt) == nil {
				log.Error("No plugin registered for the specified extension", "extension", selectedExt)
				return cli.Exit("no plugin registered for extension: "+selectedExt, 1)
			}

			log.Debug("Plugin found for extension, limiting search to this extension", "extension", selectedExt)
			exts = []string{selectedExt}
		} else {
			log.Debug("No extension specified, will search all available extensions", "extensions", exts)
		}

		log.Debug("Searching for script file", "script", name, "extensions", exts)
		scriptFile, err := util.GetScriptPathFromName(name, exts)
		if err != nil {
			log.Error("Failed to find script file", "script", name, "extensions", exts, "error", err)
			return err
		}

		log.Info("Script file found", "script", name, "path", scriptFile)

		log.Debug("Attempting to open script file with editor", "path", scriptFile)
		if err := util.OpenWithEditor(scriptFile); err != nil {
			log.Error("Failed to open script file with editor", "path", scriptFile, "error", err)
			return cli.Exit("Failed to open editor: "+err.Error(), 1)
		}

		log.Debug("Script edit command completed successfully")

		return nil
	},
}
