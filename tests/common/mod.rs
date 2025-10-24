use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Test fixture for creating temporary script and plugin directories
pub struct TestEnv {
    pub temp_dir: TempDir,
    pub script_dir: PathBuf,
    pub plugin_dir: PathBuf,
}

impl TestEnv {
    /// Create a new test environment with temporary directories
    pub fn new() -> Result<Self> {
        let temp_dir = TempDir::new()?;
        let script_dir = temp_dir.path().join("scripts");
        let plugin_dir = temp_dir.path().join("plugin");

        fs::create_dir_all(&script_dir)?;
        fs::create_dir_all(&plugin_dir)?;

        Ok(Self {
            temp_dir,
            script_dir,
            plugin_dir,
        })
    }

    /// Create a script file in the test environment
    pub fn create_script(&self, name: &str, content: &str) -> Result<PathBuf> {
        let path = self.script_dir.join(name);
        fs::write(&path, content)?;
        Ok(path)
    }

    /// Create a plugin YAML file
    pub fn create_plugin_yaml(&self, name: &str, yaml_content: &str) -> Result<PathBuf> {
        let path = self.plugin_dir.join(format!("{}.yaml", name));
        fs::write(&path, yaml_content)?;
        Ok(path)
    }

    /// Get the path to the script directory
    pub fn script_path(&self) -> &Path {
        &self.script_dir
    }

    /// Get the path to the plugin directory
    pub fn plugin_path(&self) -> &Path {
        &self.plugin_dir
    }

    /// Get the plugin database path
    pub fn plugin_db_path(&self) -> PathBuf {
        self.plugin_dir.join("plugin.dat")
    }
}

/// Sample plugin YAML for testing
pub fn sample_python_plugin() -> &'static str {
    r#"
name: python
executor: python3
arg_template:
  - "{file}"
extensions:
  - py
description: Python 3 interpreter
author: Test Author
version: 1.0.0
requires:
  - python3
"#
}

/// Sample shell plugin YAML for testing
pub fn sample_shell_plugin() -> &'static str {
    r#"
name: bash
executor: bash
arg_template:
  - "{file}"
extensions:
  - sh
  - bash
description: Bash shell interpreter
author: Test Author
version: 1.0.0
"#
}

/// Sample plugin with no extension requirements
pub fn sample_generic_plugin() -> &'static str {
    r#"
name: generic
executor: sh
arg_template:
  - "-c"
  - "chmod +x {file} && {file}"
extensions:
  - ""
description: Generic executable runner
author: Test Author
version: 1.0.0
"#
}

/// Sample script content
pub fn sample_python_script() -> &'static str {
    r#"#!/usr/bin/env python3
print("Hello from Python!")
"#
}

pub fn sample_shell_script() -> &'static str {
    r#"#!/bin/bash
echo "Hello from Shell!"
"#
}
