package main

import (
	"fmt"

	"gopkg.in/yaml.v3"
)

type PluginMeta struct {
	Name string   `yaml:"name"`
	Exts []string `yaml:"ext"`
}

func main() {
	yml := "\nname: Python Plugin\next:\n  - py\n  - pyw\n"
	var m PluginMeta
	err := yaml.Unmarshal([]byte(yml), &m)
	if err != nil {
		panic(err)
	}
	fmt.Println(m.Name) // OK
	fmt.Println(m.Exts) // [py pyw]
}
