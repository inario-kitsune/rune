package main

import (
	"context"
	"os"

	"github.com/charmbracelet/log"
	"github.com/inario-kitsune/rune/cmd"
	"github.com/inario-kitsune/rune/util"
	"github.com/urfave/cli/v3"
)

func init() {
	log.SetOutput(os.Stderr)
	log.SetLevel(log.InfoLevel)
	if level := os.Getenv("RUNE_LOG"); level != "" {
		switch level {
		case "error":
			log.SetLevel(log.ErrorLevel)
		case "warn":
			log.SetLevel(log.WarnLevel)
		case "info":
			log.SetLevel(log.InfoLevel)
		case "debug":
			log.SetLevel(log.DebugLevel)
		}

	}

}
func main() {
	app := &cli.Command{
		Name:                  "rune",
		Version:               "0.2.0",
		Usage:                 "Universal script runner with Lua plugin engine",
		EnableShellCompletion: true,
		Commands: []*cli.Command{
			cmd.PluginCommand,
			cmd.ScriptCommand,
			cmd.RunCommand,
		},
	}
	if err := util.EnsureRuntimeDirs(); err != nil {
		log.Fatalf("Failed to prepare directories: %v", err)
	}
	if err := app.Run(context.Background(), os.Args); err != nil {
		log.Fatal(err)
	}
}
