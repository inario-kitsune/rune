package script

import (
	"context"
	"fmt"
	"os"

	"github.com/charmbracelet/log"
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
		log.Info("Starting script creation", "command", "new", "filename", name)

		if name == "" {
			log.Error("Script creation failed: missing filename")
			return cli.Exit("Missing name", 1)
		}

		scriptPath := util.GetScriptPath()
		path := fmt.Sprintf("%s/%s", scriptPath, name)
		log.Debug("Script path resolved", "scriptPath", scriptPath, "fullPath", path)

		if _, err := os.Stat(path); err == nil {
			log.Warn("Script creation failed: file already exists", "path", path)
			return cli.Exit("Script already exists", 1)
		}

		log.Info("Opening script file in editor", "path", path)
		err := util.OpenWithEditor(path)
		if err != nil {
			log.Error("Failed to open editor", "error", err, "path", path)
			return err
		}

		log.Info("Script creation completed successfully", "path", path)
		log.Debug("Script creation command completed successfully")

		return nil
	},
}
