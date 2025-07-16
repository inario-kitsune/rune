package plugin

import (
	"slices"
)

type BuiltinPlugin struct {
	PluginName string
	Exts       []string
	Cmd        func(target string, args []string) error
}

func NewBuiltinPlugin(name string, exts []string, exec func(target string, args []string) error) *BuiltinPlugin {
	return &BuiltinPlugin{name, exts, exec}

}
func (p *BuiltinPlugin) Name() string {
	return p.PluginName
}
func (p *BuiltinPlugin) GetPath() string {
	return "builtin"
}
func (p *BuiltinPlugin) Supports(ext string) bool {
	return slices.Contains(p.Exts, ext)
}
func (p *BuiltinPlugin) Run(target string, args []string) error {
	return p.Cmd(target, args)
}
