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
		var sourceFile = c.Args().First()
		log.Info(fmt.Sprintf("try to install %s", sourceFile))
		if !plugin.Validate(sourceFile) {
			log.Error(fmt.Sprintf("Plugin %s invalidate", sourceFile), "sourceFile", sourceFile)
			return fmt.Errorf("Plugin %s invalidate", sourceFile)
		}
		log.Info("%s validated", sourceFile)
		pluginPath := util.GetPluginPath()
		targetPath := filepath.Join(pluginPath, filepath.Base(sourceFile))
		if _, err := os.Stat(targetPath); err == nil {
			log.Error("Plugin already exists")
			return cli.Exit("Plugin already exists", 1)
		}
		log.Info(fmt.Sprintf("installing %s", sourceFile))
		return os.Rename(sourceFile, targetPath)

	},
}
