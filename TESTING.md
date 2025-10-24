# Rune Testing Guide

This document describes the testing strategy for the Rune project.

## Overview

Rune uses a comprehensive testing approach with:
- **Unit tests**: Testing individual functions and modules in isolation
- **Integration tests**: Testing complete workflows (planned)
- **Test fixtures**: Reusable test utilities and sample data

## Running Tests

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run specific test
cargo test test_load_scripts

# Run with output
cargo test -- --nocapture

# Run tests in quiet mode
cargo test --quiet
```

## Test Coverage

### Core Module Tests

#### `core/script.rs` (8 tests)
Tests for script loading and management:
- ✅ `test_load_scripts_with_various_extensions` - Load scripts with .sh, .py, .rb, .R extensions
- ✅ `test_load_scripts_empty_directory` - Handle empty directories gracefully
- ✅ `test_load_scripts_ignores_directories` - Only load files, not subdirectories
- ✅ `test_load_scripts_ignores_files_without_extensions` - Filter out extension-less files
- ✅ `test_load_scripts_duplicate_names_different_extensions` - Handle convert.py, convert.sh, convert.rb
- ✅ `test_load_scripts_nonexistent_directory` - Error handling for missing directories
- ✅ `test_script_path_preservation` - Verify absolute paths are preserved

#### `core/plugin.rs` (20 tests)
Tests for plugin parsing and database operations:

**Plugin YAML Parsing:**
- ✅ `test_plugin_from_yaml_valid` - Parse complete plugin definition
- ✅ `test_plugin_from_yaml_minimal` - Parse minimal plugin with defaults
- ✅ `test_plugin_from_yaml_missing_name` - Reject empty plugin names
- ✅ `test_plugin_from_yaml_missing_executor` - Reject empty executors
- ✅ `test_plugin_from_yaml_missing_extensions` - Reject plugins without extensions
- ✅ `test_plugin_from_yaml_invalid_yaml` - Handle malformed YAML
- ✅ `test_plugin_to_yaml` - Serialize plugins to YAML
- ✅ `test_plugin_yaml_roundtrip` - Verify parse→serialize→parse consistency
- ✅ `test_plugin_validate_always_succeeds` - Validation only warns, doesn't fail

**Plugin Database:**
- ✅ `test_plugin_database_new` - Create empty database
- ✅ `test_plugin_database_add_plugin` - Add new plugins
- ✅ `test_plugin_database_add_duplicate` - Reject duplicate plugin names
- ✅ `test_plugin_database_update_plugin` - Update existing plugins
- ✅ `test_plugin_database_remove_plugin` - Remove plugins
- ✅ `test_plugin_database_remove_nonexistent` - Error on missing plugins
- ✅ `test_plugin_database_get_plugin` - Retrieve plugins by name
- ✅ `test_plugin_database_find_by_extension_single` - Find one matching plugin
- ✅ `test_plugin_database_find_by_extension_multiple` - Find multiple, sorted by name
- ✅ `test_plugin_database_find_by_extension_none` - Handle no matches
- ✅ `test_plugin_database_all_plugins` - Iterate all plugins

#### `core/executor.rs` (8 tests)
Tests for command execution:
- ✅ `test_command_executor_new` - Create executor
- ✅ `test_command_executor_arg` - Add single argument
- ✅ `test_command_executor_args_chaining` - Chain multiple arguments
- ✅ `test_command_executor_args_vec` - Add vector of arguments
- ✅ `test_command_executor_check_available_valid` - Verify `echo` exists
- ✅ `test_command_executor_check_available_invalid` - Detect missing commands
- ✅ `test_command_executor_execute_simple` - Execute basic command
- ✅ `test_command_executor_execute_nonexistent` - Error on missing command
- ✅ `test_command_executor_execute_failing_command` - Detect non-zero exit codes

## Test Utilities

### `tests/common/mod.rs`
Shared test utilities (ready for integration tests):

```rust
// Create temporary test environment
let env = TestEnv::new()?;

// Create test scripts
env.create_script("backup.sh", "#!/bin/bash\necho test")?;

// Create plugin definitions
env.create_plugin_yaml("python", "name: python\n...")?;

// Sample plugin definitions
sample_python_plugin()
sample_shell_plugin()
sample_generic_plugin()
```

## Test Statistics

- **Total Tests**: 36
- **Pass Rate**: 100%
- **Coverage**: Core business logic fully tested
- **Integration Tests**: Planned (see Future Work)

## Design Decisions

### Why Extension-less Files Are Excluded
Rune is designed as a **centralized script runner**, not a project build tool:
- ✅ **Included**: Portable scripts (.sh, .py, .rb) that can run anywhere
- ❌ **Excluded**: Project-specific files (Makefile, Dockerfile) tied to directories

This is validated by `test_load_scripts_ignores_files_without_extensions`.

### Testing Philosophy
1. **Unit tests first**: Test core logic in isolation
2. **Mock external dependencies**: Use temp directories instead of real filesystem
3. **Cross-platform**: Tests work on Windows, macOS, Linux
4. **Fast execution**: All 36 tests run in < 1 second

## Future Work

### Integration Tests (Planned)
Create `tests/integration_test.rs` for end-to-end workflows:
- [ ] Add script → List → Run → Remove workflow
- [ ] Plugin add → Info → Export → Remove workflow
- [ ] Script execution with different plugins
- [ ] Error scenarios (missing plugins, invalid scripts)
- [ ] CLI argument parsing

### Additional Unit Tests
- [ ] `utils/fs.rs` - Path resolution tests
- [ ] `utils/cli.rs` - User prompt tests
- [ ] `commands/run.rs` - Run command logic
- [ ] `commands/script.rs` - Script management commands
- [ ] `commands/plugin.rs` - Plugin management commands

### Test Coverage Tools
```bash
# Install tarpaulin for coverage reports
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

## Contributing Tests

When adding new features:
1. Write tests **before** implementing the feature (TDD)
2. Ensure tests are **deterministic** (no random data)
3. Use **descriptive test names** (test_what_when_expected)
4. Add **comments** explaining complex test scenarios
5. Keep tests **focused** (one concept per test)
6. Run `cargo test` before committing

## Example Test Template

```rust
#[test]
fn test_feature_name_scenario() {
    // Arrange: Set up test data
    let input = create_test_data();

    // Act: Execute the function under test
    let result = function_under_test(input);

    // Assert: Verify expected outcomes
    assert_eq!(result, expected_value);
    assert!(result.is_ok());
}
```
