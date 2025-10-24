package util

import (
	"os"
	"path/filepath"
	"runtime"
)

func getDataHome() string {
	if runtime.GOOS == "windows" {
		if appdata := os.Getenv("APPDATA"); appdata != "" {
			return filepath.Join(appdata, "rune")
		}
		return filepath.Join(os.Getenv("USERPROFILE"), "AppData", "Roaming", "rune")
	}
	if xdg := os.Getenv("XDG_DATA_HOME"); xdg != "" {
		return filepath.Join(xdg, "rune")
	}
	home, _ := os.UserHomeDir()
	return filepath.Join(home, ".local", "share", "rune")
}
func GetPluginPath() string {
	if path := os.Getenv("RUNE_PLUGIN"); path != "" {
		return path
	}
	return filepath.Join(getDataHome(), "plugins")
}
func GetScriptPath() string {
	if path := os.Getenv("RUNE_REPO"); path != "" {
		return path
	}
	return filepath.Join(getDataHome(), "scripts")
}
func EnsureRuntimeDirs() error {
	for _, dir := range []string{GetScriptPath(), GetPluginPath()} {
		if err := os.MkdirAll(dir, 0755); err != nil {
			return err
		}
	}
	return nil
}
