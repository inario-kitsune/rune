package util

import (
	"fmt"
	"io/fs"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

func FilterFilesByExt(dir string, exts []string) ([]string, error) {
	extSet := make(map[string]struct{}, len(exts))
	for _, ext := range exts {
		ext = strings.ToLower(strings.TrimPrefix(ext, "."))
		extSet[ext] = struct{}{}
	}
	var matches []string
	err := filepath.WalkDir(dir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if d.IsDir() {
			return nil
		}
		ext := strings.ToLower(strings.TrimPrefix(filepath.Ext(d.Name()), "."))
		if _, ok := extSet[ext]; ok {
			matches = append(matches, path)
		}
		return nil
	})
	return matches, err
}
func OpenWithEditor(path string) error {
	editor := os.Getenv("EDITOR")
	if editor == "" {
		editor = "nano"
	}
	cmd := exec.Command(editor, path)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	if err := cmd.Run(); err != nil {
		return fmt.Errorf("failed to open editor %q: %w", editor, err)
	}
	return nil
}
func GetScriptPathFromName(name string, extensions []string) (string, error) {
	scriptPath := GetScriptPath()
	scripts, err := FilterFilesByExt(scriptPath, extensions)
	if err != nil {
		return "", err
	}
	scriptMap := make(map[string]string)
	for _, path := range scripts {
		base := filepath.Base(path)
		ext := filepath.Ext(path)
		baseName := strings.TrimSuffix(base, ext)
		scriptMap[baseName] = path
	}
	scriptFile, ok := scriptMap[name]
	if !ok {
		return "", fmt.Errorf("script not found: %s", name)
	}
	return scriptFile, nil
}
