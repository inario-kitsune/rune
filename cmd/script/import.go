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
		log.Debug("Starting script import command")

		sourceFile := c.Args().First()
		log.Debug("Retrieved source file from arguments", "source_file", sourceFile)

		if sourceFile == "" {
			log.Error("Source file path is required but not provided")
			return cli.Exit("Missing source file path", 1)
		}

		log.Info("Starting to import script file", "source", sourceFile)

		log.Debug("Checking if source file exists", "path", sourceFile)
		if _, err := os.Stat(sourceFile); err != nil {
			if os.IsNotExist(err) {
				log.Error("Source file does not exist", "path", sourceFile)
				return cli.Exit("Source file not found", 1)
			}
			log.Error("Failed to check source file status", "path", sourceFile, "error", err)
			return cli.Exit(fmt.Sprintf("Error checking source file: %v", err), 1)
		}
		log.Debug("Source file exists and is accessible", "path", sourceFile)

		scriptPath := util.GetScriptPath()
		log.Debug("Retrieved script path", "script_path", scriptPath)

		targetPath := filepath.Join(scriptPath, filepath.Base(sourceFile))
		log.Debug("Constructed target path", "target_path", targetPath, "filename", filepath.Base(sourceFile))

		log.Debug("Checking if target file already exists", "path", targetPath)
		if _, err := os.Stat(targetPath); err == nil {
			log.Error("Script file already exists in target location",
				"source", sourceFile,
				"target", targetPath,
				"filename", filepath.Base(sourceFile))
			return cli.Exit("Script already exists", 1)
		} else if !os.IsNotExist(err) {
			log.Error("Failed to check target file status", "path", targetPath, "error", err)
			return cli.Exit(fmt.Sprintf("Error checking target file: %v", err), 1)
		}
		log.Debug("Target path is available", "path", targetPath)

		log.Info("Moving script file to target location",
			"source", sourceFile,
			"target", targetPath)

		// 执行文件移动
		log.Debug("Attempting to move file", "from", sourceFile, "to", targetPath)
		if err := os.Rename(sourceFile, targetPath); err != nil {
			log.Error("Failed to move script file",
				"source", sourceFile,
				"target", targetPath,
				"error", err)
			return cli.Exit(fmt.Sprintf("Failed to import script: %v", err), 1)
		}

		log.Info("Successfully imported script file",
			"filename", filepath.Base(sourceFile),
			"from", sourceFile,
			"to", targetPath)
		log.Debug("Script import command completed successfully")

		return nil
	},
}
