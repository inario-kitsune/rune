package script

import (
	"context"
	"os"
	"path/filepath"

	"github.com/inario-kitsune/rune/util"
	"github.com/urfave/cli/v3"
)

var ScriptImportCommand = &cli.Command{
	Name:      "import",
	Aliases:   []string{"i"},
	Usage:     "import a script file",
	ArgsUsage: "<Script file>",
	Action: func(ctx context.Context, c *cli.Command) error {
		var sourceFile = c.Args().First()
		scriptPath := util.GetScriptPath()
		targetPath := filepath.Join(scriptPath, filepath.Base(sourceFile))
		if _, err := os.Stat(targetPath); err == nil {
			return cli.Exit("Script already exists", 1)
		}
		return os.Rename(sourceFile, targetPath)

	},
}
