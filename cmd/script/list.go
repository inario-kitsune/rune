package script

import (
	"context"
	"fmt"
	"path/filepath"
	"strconv"
	"strings"

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
		scriptPath := util.GetScriptPath()
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
		scripts, err := util.FilterFilesByExt(scriptPath, exts)
		if err != nil {
			return err
		}
		if !c.Bool("plain") {
			header := []string{"ID", "Name", "Extension"}
			var rows [][]string
			for index, script := range scripts {
				base := filepath.Base(script)
				ext := filepath.Ext(script)
				base = strings.TrimSuffix(base, ext)
				ext = strings.ToLower(strings.TrimPrefix(ext, "."))
				rows = append(rows, []string{
					strconv.Itoa(index + 1),
					base,
					ext,
				})
			}
			util.PrintTable(header, rows)
		} else {
			for _, script := range scripts {
				base := strings.TrimSuffix(filepath.Base(script), filepath.Ext(script))
				fmt.Printf("%s\n", base)

			}
		}

		return nil
	},
}
