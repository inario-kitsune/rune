package plugin

import (
	"context"
	"fmt"
	"path/filepath"
	"strings"

	"github.com/charmbracelet/log"
	"github.com/inario-kitsune/rune/plugin"
	"github.com/inario-kitsune/rune/util"
	"github.com/urfave/cli/v3"
)

var PluginListCommand = &cli.Command{
	Name:    "list",
	Aliases: []string{"ls"},
	Usage:   "List available plugins",
	Flags: []cli.Flag{
		&cli.BoolFlag{
			Name:    "plain",
			Aliases: []string{"p"},
			Usage:   "List one file per line (no formatting)",
		},
	},
	Action: func(ctx context.Context, c *cli.Command) error {
		log.Debug("Starting plugin list command")

		plainMode := c.Bool("plain")
		log.Debug("Command flags parsed", "plain_mode", plainMode)

		pluginPath := util.GetPluginPath()
		log.Debug("Retrieved plugin path", "path", pluginPath)

		log.Info("Loading plugins from directory", "path", pluginPath)
		err := plugin.LoadPlugins(pluginPath)
		if err != nil {
			log.Error("Failed to load plugins", "path", pluginPath, "error", err)
			return err
		}
		log.Debug("Successfully loaded plugins")

		plugins := plugin.GetPluginList()
		pluginCount := len(plugins)
		log.Info("Retrieved plugin list", "total_plugins", pluginCount)

		if pluginCount == 0 {
			log.Warn("No plugins found")
			if plainMode {
				log.Debug("Plain mode enabled, but no plugins to display")
			} else {
				log.Debug("Table mode enabled, but no plugins to display")
			}
			return nil
		}

		if plainMode {
			log.Debug("Using plain output mode")

			nameSet := make(map[string]struct{}, pluginCount)
			log.Debug("Creating unique plugin name set", "initial_capacity", pluginCount)

			for ext, plugin := range plugins {
				name := strings.TrimSuffix(filepath.Base(plugin.Path), ".lua")
				nameSet[name] = struct{}{}
				log.Debug("Added plugin to name set",
					"extension", ext,
					"plugin_name", plugin.Name,
					"file_name", name,
					"path", plugin.Path)
			}

			uniqueCount := len(nameSet)
			log.Debug("Unique plugin names collected", "unique_count", uniqueCount)

			log.Info("Outputting plugin names in plain mode", "count", uniqueCount)
			for name := range nameSet {
				fmt.Printf("%s\n", name)
				log.Debug("Output plugin name", "name", name)
			}

		} else {
			log.Debug("Using table output mode")

			header := []string{"Extension", "Name", "File"}
			var rows [][]string

			log.Debug("Building table data", "header", header)

			builtinCount := 0
			userPluginCount := 0

			for ext, plugin := range plugins {
				source := "builtin:"
				if !strings.HasPrefix(plugin.Path, "builtin:") {
					source = ""
					userPluginCount++
				} else {
					builtinCount++
				}

				fileName := source + filepath.Base(plugin.Path)
				row := []string{ext, plugin.Name, fileName}
				rows = append(rows, row)

				log.Debug("Added plugin to table",
					"extension", ext,
					"plugin_name", plugin.Name,
					"file_name", fileName,
					"is_builtin", strings.HasPrefix(plugin.Path, "builtin:"),
					"path", plugin.Path)
			}

			log.Info("Table data prepared",
				"total_rows", len(rows),
				"builtin_plugins", builtinCount,
				"user_plugins", userPluginCount)

			log.Debug("Outputting plugin table")
			util.PrintTable(header, rows)
		}

		log.Debug("Plugin list command completed successfully")
		return nil
	},
}
