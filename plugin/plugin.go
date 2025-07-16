package plugin

import (
	"fmt"
	"slices"

	lua "github.com/yuin/gopher-lua"
)

type IPlugin interface {
	Name() string
	Supports(string) bool
	Run(target string, args []string) error
	GetPath() string
}

type LuaPlugin struct {
	PluginName string   `yaml:"name"`
	Exts       []string `yaml:"ext"`
	Path       string   `yaml:"-"`
}

func NewLuaPlugin(name string, exts []string, path string) *LuaPlugin {
	return &LuaPlugin{name, exts, path}
}
func (p *LuaPlugin) Name() string {
	return p.PluginName
}
func (p *LuaPlugin) Supports(ext string) bool {
	return slices.Contains(p.Exts, ext)
}
func (p *LuaPlugin) GetPath() string {
	return p.Path
}
func (p *LuaPlugin) Run(target string, args []string) error {
	L := lua.NewState()
	defer L.Close()

	L.SetGlobal("target", lua.LString(target))

	argTable := L.NewTable()
	for _, a := range args {
		argTable.Append(lua.LString(a))
	}
	L.SetGlobal("args", argTable)

	if err := L.DoFile(p.Path); err != nil {
		return fmt.Errorf("lua plugin error %w", err)
	}
	return nil
}
