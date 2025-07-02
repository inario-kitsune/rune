package script

import (
	"context"
	"fmt"
	"os"

	"github.com/inario-kitsune/rune/util"
	"github.com/urfave/cli/v3"
)

var ScriptNewCommand = &cli.Command{
	Name:      "new",
	Aliases:   []string{"n"},
	Usage:     "Create a new Script",
	ArgsUsage: "<filename>",
	Action: func(ctx context.Context, c *cli.Command) error {
		name := c.Args().First()
		if name == "" {
			return cli.Exit("Missing name", 1)
		}
		scriptPath := util.GetScriptPath()
		path := fmt.Sprintf("%s/%s", scriptPath, name)
		if _, err := os.Stat(path); err == nil {
			return cli.Exit("Script already exists", 1)
		}
		return util.OpenWithEditor(path)
	},
}
