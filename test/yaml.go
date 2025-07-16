package main

import (
	"fmt"

	"gopkg.in/yaml.v3"
)

type PluginMeta struct {
	PluginName string   `yaml:"name"`
	Exts       []string `yaml:"ext"`
	Path       string   `yaml:"-"`
}

func main() {
	yml := "\nname: Python Plugin\next:\n  - py\n  - pyw\n"
	var m PluginMeta
	err := yaml.Unmarshal([]byte(yml), &m)
	if err != nil {
		panic(err)
	}
	fmt.Println(m.PluginName) // OK
	fmt.Println(m.Exts)       // [py pyw]
}
