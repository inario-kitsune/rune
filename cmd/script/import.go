package script

import (
	"context"
	"fmt"
	"os"
	"path/filepath"

	"github.com/charmbracelet/log"
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
		log.Info(fmt.Sprintf("try to import %s", sourceFile))
		scriptPath := util.GetScriptPath()
		targetPath := filepath.Join(scriptPath, filepath.Base(sourceFile))
		if _, err := os.Stat(targetPath); err == nil {
			log.Error("Script already exists")
			return cli.Exit("Script already exists", 1)
		}
		log.Info(fmt.Sprintf("importing %s", sourceFile))
		return os.Rename(sourceFile, targetPath)

	},
}
