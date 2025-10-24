package plugin

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"regexp"
	"strings"

	lua "github.com/yuin/gopher-lua"
	"gopkg.in/yaml.v3"
)

var pluginMap = map[string]*Plugin{}

var metaBlockRegex = regexp.MustCompile(`(?s)--\[\[\s*rune-meta(.*?)\]\]`)

func LoadPlugins(dir string) error {
	if err := loadEmbeddedPlugins(); err != nil {
		return fmt.Errorf("failed to load embedded plugins: %w", err)
	}
	return filepath.WalkDir(dir, func(path string, d fs.DirEntry, err error) error {
		if err != nil || d.IsDir() || !strings.HasSuffix(path, ".lua") {
			return nil
		}
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
func extractMeta(path string) (*Plugin, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	matches := metaBlockRegex.FindSubmatch(data)
	if matches == nil {
		return nil, errors.New("missing rune-meta block")
	}
	var p Plugin
	if err := yaml.Unmarshal(matches[1], &p); err != nil {
		return nil, fmt.Errorf("yaml error: %w", err)
	}
	p.Execute = func(targetScript string, args []string) error {
		L := lua.NewState()
		defer L.Close()
		L.SetGlobal("target", lua.LString(targetScript))
		luaArgs := L.NewTable()
		for i, arg := range args {
			L.RawSet(luaArgs, lua.LNumber(i+1), lua.LString(arg))
		}
		L.SetGlobal("args", luaArgs)
		return L.DoFile(p.Path)
	}
	return &p, nil
}

func GetPluginByExt(ext string) *Plugin {
	return pluginMap[ext]
}
func GetPluginList() map[string]*Plugin {
	return pluginMap
}

func registerPluginFromBytes(content []byte, virtualPath string) error {
	matches := metaBlockRegex.FindSubmatch(content)
	if matches == nil {
		return fmt.Errorf("missing rune-meta block in %s", virtualPath)
	}

	var p Plugin
	if err := yaml.Unmarshal(matches[1], &p); err != nil {
		return fmt.Errorf("yaml error in %s: %w", virtualPath, err)
	}
	// 设置 Path 字段为虚拟路径（可选）
	p.Path = "builtin:" + virtualPath

	// 自定义 Execute 使用内存代码运行
	code := string(content)
	p.Execute = func(targetScript string, args []string) error {
		L := lua.NewState()
		defer L.Close()
		L.SetGlobal("target", lua.LString(targetScript))
		luaArgs := L.NewTable()
		for i, arg := range args {
			L.RawSet(luaArgs, lua.LNumber(i+1), lua.LString(arg))
		}
		L.SetGlobal("args", luaArgs)
		return L.DoString(code)
	}

	for _, ext := range p.Exts {
		if _, ok := pluginMap[ext]; !ok {
			pluginMap[ext] = &p
		}
	}
	return nil
}
