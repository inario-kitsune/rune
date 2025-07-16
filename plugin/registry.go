package plugin

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"strings"

	"github.com/charmbracelet/log"
	"gopkg.in/yaml.v3"
)

var pluginMap = map[string]IPlugin{}

var metaBlockRegex = regexp.MustCompile(`(?s)--\[\[\s*rune-meta(.*?)\]\]`)

func LoadPlugins(dir string) error {
	if err := loadEmbeddedPlugins(); err != nil {
		return fmt.Errorf("failed to load embedded plugins: %w", err)
	}
	return filepath.WalkDir(dir, func(path string, d fs.DirEntry, err error) error {
		if err != nil || d.IsDir() || !strings.HasSuffix(path, ".lua") {
			return nil
		}
		log.Debug("loading plugin file", "path", path)
		plugin, err := extractMeta(path)
		if err != nil {
			return fmt.Errorf("plugin %s error: %w", path, err)
		}
		plugin.Path = path
		for _, ext := range plugin.Exts {
			pluginMap[ext] = plugin
		}
		return nil
	})
}
func extractMeta(path string) (*LuaPlugin, error) {
	data, err := os.ReadFile(path)
	log.Debug("plugin metadata", "data", string(data))

	if err != nil {
		return nil, err
	}
	matches := metaBlockRegex.FindSubmatch(data)
	if matches == nil {
		return nil, errors.New("missing rune-meta block")
	}
	log.Debug("plugin metadata", "matches", string(matches[1]))
	var p LuaPlugin
	if err := yaml.Unmarshal(matches[1], &p); err != nil {
		return nil, fmt.Errorf("yaml error: %w", err)
	}
	log.Debug("loading plugin", "plugin.name", p.Name(), "plugin.name", p.Exts)
	return &p, nil
}

func GetPluginByExt(ext string) IPlugin {
	return pluginMap[ext]
}
func GetPluginList() map[string]IPlugin {
	return pluginMap
}
func loadEmbeddedPlugins() error {
	pluginMap["lua"] = NewBuiltinPlugin("lua", []string{"lua"}, func(target string, args []string) error {
		allArgs := append([]string{target}, args...)
		cmd := exec.Command("lua", allArgs...)
		cmd.Stderr = cmd.Stdout
		out, err := cmd.CombinedOutput()
		fmt.Println(string(out))
		return err
	})
	return nil
}
