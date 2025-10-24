package plugin

import (
	"embed"
	"io/fs"
)

//go:embed plugins/*.lua
var embededPlugins embed.FS

func loadEmbeddedPlugins() error {
	return fs.WalkDir(embededPlugins, "plugins", func(path string, d fs.DirEntry, err error) error {
		if err != nil || d.IsDir() {
			return nil
		}
		content, err := embededPlugins.ReadFile(path)
		if err != nil {
			return err
		}
		return registerPluginFromBytes(content, path)
	})
}
