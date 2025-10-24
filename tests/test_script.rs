mod common;

use anyhow::Result;
use common::TestEnv;

// We need to add a test module inside the main crate for internal testing
// This integration test file tests the public API

#[test]
fn test_load_scripts_with_extensions() -> Result<()> {
    let env = TestEnv::new()?;

    // Create scripts with different extensions
    env.create_script("backup.sh", common::sample_shell_script())?;
    env.create_script("deploy.py", common::sample_python_script())?;
    env.create_script("process.rb", "#!/usr/bin/env ruby\nputs 'Hello'")?;

    // Note: We'll need to expose load_scripts as pub for testing
    // or add tests in src/core/script.rs as a #[cfg(test)] module

    Ok(())
}

#[test]
fn test_load_scripts_without_extensions() -> Result<()> {
    let env = TestEnv::new()?;

    // Create scripts without extensions (common in Unix)
    env.create_script("Makefile", "all:\n\techo 'Building...'")?;
    env.create_script("Dockerfile", "FROM ubuntu:latest")?;
    env.create_script("build", "#!/bin/bash\necho 'Building...'")?;

    // These should also be loaded after we fix the bug

    Ok(())
}

#[test]
fn test_empty_script_directory() -> Result<()> {
    let env = TestEnv::new()?;

    // Script directory exists but is empty
    // Should return empty Vec, not error

    Ok(())
}

#[test]
fn test_script_directory_with_subdirectories() -> Result<()> {
    let env = TestEnv::new()?;

    // Create a script and a subdirectory
    env.create_script("script.sh", common::sample_shell_script())?;
    std::fs::create_dir_all(env.script_path().join("subdir"))?;

    // Should only load files, not directories

    Ok(())
}
