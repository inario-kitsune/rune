package script

import (
	"context"
	"fmt"
	"path/filepath"
	"strconv"
	"strings"

	"github.com/charmbracelet/log"
	"github.com/inario-kitsune/rune/plugin"
	"github.com/inario-kitsune/rune/util"
	"github.com/urfave/cli/v3"
)

var ScriptListCommand = &cli.Command{
	Name:    "list",
	Aliases: []string{"ls"},
	Usage:   "List available plugins",
	Flags: []cli.Flag{
		&cli.BoolFlag{
			Name:    "plain",
			Aliases: []string{"p"},
			Usage:   "List one file per line (no formatting)",
		},
		&cli.StringFlag{
			Name:    "extension",
			Aliases: []string{"x"},
			Usage:   "Force script extension (e.g. py,lua)",
		},
	},
	Action: func(ctx context.Context, c *cli.Command) error {
		log.Debug("Starting script list command")

		scriptPath := util.GetScriptPath()
		pluginPath := util.GetPluginPath()
		log.Debug("Retrieved paths", "script_path", scriptPath, "plugin_path", pluginPath)

		log.Debug("Loading plugins", "plugin_path", pluginPath)
		if err := plugin.LoadPlugins(pluginPath); err != nil {
			log.Error("Failed to load plugins", "plugin_path", pluginPath, "error", err)
			return err
		}
		log.Info("Successfully loaded plugins", "plugin_path", pluginPath)

		exts := util.Keys(plugin.GetPluginList())
		log.Debug("Retrieved available extensions", "extensions", exts, "count", len(exts))

		selectedExt := c.String("extension")
		if selectedExt != "" {
			log.Debug("Processing extension filter", "raw_extension", selectedExt)
			selectedExt = strings.ToLower(strings.TrimPrefix(selectedExt, "."))
			log.Debug("Normalized extension", "extension", selectedExt)

			if plugin.GetPluginByExt(selectedExt) == nil {
				log.Error("No plugin registered for extension", "extension", selectedExt)
				return cli.Exit("no plugin registered for extension: "+selectedExt, 1)
			}

			exts = []string{selectedExt}
			log.Info("Filtered to specific extension", "extension", selectedExt)
		} else {
			log.Debug("No extension filter specified, using all available extensions")
		}

		log.Debug("Searching for script files", "script_path", scriptPath, "extensions", exts)
		scripts, err := util.FilterFilesByExt(scriptPath, exts)
		if err != nil {
			log.Error("Failed to filter files by extension", "script_path", scriptPath, "extensions", exts, "error", err)
			return err
		}
		log.Info("Found script files", "count", len(scripts), "extensions", exts)

		if len(scripts) == 0 {
			log.Warn("No script files found", "script_path", scriptPath, "extensions", exts)
		}

		plainFormat := c.Bool("plain")
		log.Debug("Output format determined", "plain", plainFormat)

		if !plainFormat {
			log.Debug("Preparing table output")
			header := []string{"ID", "Name", "Extension"}
			var rows [][]string

			for index, script := range scripts {
				base := filepath.Base(script)
				ext := filepath.Ext(script)
				base = strings.TrimSuffix(base, ext)
				ext = strings.ToLower(strings.TrimPrefix(ext, "."))

				row := []string{
					strconv.Itoa(index + 1),
					base,
					ext,
				}
				rows = append(rows, row)

				log.Debug("Added script to table",
					"index", index+1,
					"name", base,
					"extension", ext,
					"full_path", script)
			}

			log.Debug("Displaying table output", "rows", len(rows))
			util.PrintTable(header, rows)

		} else {
			log.Debug("Displaying plain output")
			for index, script := range scripts {
				base := strings.TrimSuffix(filepath.Base(script), filepath.Ext(script))
				fmt.Printf("%s\n", base)
				log.Debug("Listed script", "index", index+1, "name", base, "full_path", script)
			}
		}

		log.Debug("Script list command completed successfully")
		return nil
	},
}
