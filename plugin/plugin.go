package plugin

import (
	"fmt"
	"os"

	lua "github.com/yuin/gopher-lua"
)

type Plugin struct {
	Name    string                                         `yaml:"name"`
	Exts    []string                                       `yaml:"ext"`
	Path    string                                         `yaml:"-"`
	Execute func(targetScript string, args []string) error `yaml:"-"`
}

func Validate(path string) bool {
	if _, err := os.Stat(path); err != nil {
		fmt.Printf("plugin not found: %s\n", path)
		return false
	}

	L := lua.NewState()
	defer L.Close()

	L.SetGlobal("target", lua.LString("__dummy__"))
	L.SetGlobal("args", L.NewTable())
	if err := L.DoFile(path); err != nil {
		fmt.Fprintf(os.Stderr, "plugin load error (%s): %v\n", path, err)
		return false
	}
	return true
}
