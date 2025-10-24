use anyhow::{Context, Result};
use std::{fs, path::PathBuf};

/// 脚本信息结构体
#[derive(Debug)]
pub struct Script {
    /// 脚本名称（不含扩展名）
    pub name: String,
    /// 文件扩展名
    pub extension: String,
    /// 脚本文件的完整路径
    pub path: PathBuf,
}

/// 从指定目录加载所有脚本
///
/// # 参数
/// * `path` - 脚本目录路径
///
/// # 返回
/// 返回脚本列表，只包含有扩展名的文件（排除 Makefile、Dockerfile 等）
///
/// # 示例
/// ```no_run
/// use rune::core::script::load_scripts;
/// use std::path::PathBuf;
///
/// let scripts = load_scripts(PathBuf::from("~/.local/share/rune/scripts"))?;
/// for script in scripts {
///     println!("{}.{}", script.name, script.extension);
/// }
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn load_scripts(path: PathBuf) -> Result<Vec<Script>> {
    // 读取目录中的所有条目
    let entries = fs::read_dir(&path)
        .context(format!("Failed to read directory: {:?}", path))?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false));

    // 过滤并转换为 Script 对象
    let scripts: Vec<Script> = entries
        .filter_map(|entry| {
            let file_path = entry.path();
            let name = file_path.file_stem()?.to_str()?.to_string();
            // 只处理有扩展名的文件，过滤掉 Makefile、Dockerfile 等
            let extension = file_path.extension()?.to_str()?.to_string();
            Some(Script {
                name,
                extension,
                path: file_path,
            })
        })
        .collect();
    Ok(scripts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_load_scripts_with_various_extensions() {
        let temp_dir = TempDir::new().unwrap();
        let script_dir = temp_dir.path();

        // 创建不同扩展名的测试脚本
        fs::write(script_dir.join("backup.sh"), "#!/bin/bash\necho test").unwrap();
        fs::write(script_dir.join("deploy.py"), "#!/usr/bin/env python3\nprint('test')").unwrap();
        fs::write(script_dir.join("process.rb"), "#!/usr/bin/env ruby\nputs 'test'").unwrap();
        fs::write(script_dir.join("analyze.R"), "print('test')").unwrap();

        let scripts = load_scripts(script_dir.to_path_buf()).unwrap();

        assert_eq!(scripts.len(), 4);

        // 验证每个脚本的名称和扩展名
        let backup = scripts.iter().find(|s| s.name == "backup").unwrap();
        assert_eq!(backup.extension, "sh");

        let deploy = scripts.iter().find(|s| s.name == "deploy").unwrap();
        assert_eq!(deploy.extension, "py");

        let process = scripts.iter().find(|s| s.name == "process").unwrap();
        assert_eq!(process.extension, "rb");

        let analyze = scripts.iter().find(|s| s.name == "analyze").unwrap();
        assert_eq!(analyze.extension, "R");
    }

    #[test]
    fn test_load_scripts_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let script_dir = temp_dir.path();

        let scripts = load_scripts(script_dir.to_path_buf()).unwrap();

        assert_eq!(scripts.len(), 0);
    }

    #[test]
    fn test_load_scripts_ignores_directories() {
        let temp_dir = TempDir::new().unwrap();
        let script_dir = temp_dir.path();

        // 创建脚本文件和子目录
        fs::write(script_dir.join("script.sh"), "#!/bin/bash").unwrap();
        fs::create_dir_all(script_dir.join("subdir")).unwrap();
        fs::create_dir_all(script_dir.join("backup")).unwrap();

        let scripts = load_scripts(script_dir.to_path_buf()).unwrap();

        // 应该只加载文件，不加载目录
        assert_eq!(scripts.len(), 1);
        assert_eq!(scripts[0].name, "script");
        assert_eq!(scripts[0].extension, "sh");
    }

    #[test]
    fn test_load_scripts_ignores_files_without_extensions() {
        let temp_dir = TempDir::new().unwrap();
        let script_dir = temp_dir.path();

        // 创建有效的脚本
        fs::write(script_dir.join("backup.sh"), "#!/bin/bash").unwrap();
        fs::write(script_dir.join("deploy.py"), "#!/usr/bin/env python3").unwrap();

        // 创建无扩展名的文件（应该被忽略）
        fs::write(script_dir.join("README"), "Documentation").unwrap();
        fs::write(script_dir.join("Makefile"), "all:\n\techo test").unwrap();

        let scripts = load_scripts(script_dir.to_path_buf()).unwrap();

        // 只应该加载有扩展名的文件
        assert_eq!(scripts.len(), 2);
        assert!(scripts.iter().all(|s| !s.extension.is_empty()));
    }

    #[test]
    fn test_load_scripts_duplicate_names_different_extensions() {
        let temp_dir = TempDir::new().unwrap();
        let script_dir = temp_dir.path();

        // 创建同名但不同扩展名的脚本
        fs::write(script_dir.join("convert.py"), "#!/usr/bin/env python3").unwrap();
        fs::write(script_dir.join("convert.sh"), "#!/bin/bash").unwrap();
        fs::write(script_dir.join("convert.rb"), "#!/usr/bin/env ruby").unwrap();

        let scripts = load_scripts(script_dir.to_path_buf()).unwrap();

        assert_eq!(scripts.len(), 3);

        // 所有脚本应该有相同的名称
        assert!(scripts.iter().all(|s| s.name == "convert"));

        // 但有不同的扩展名
        let extensions: Vec<&str> = scripts.iter().map(|s| s.extension.as_str()).collect();
        assert!(extensions.contains(&"py"));
        assert!(extensions.contains(&"sh"));
        assert!(extensions.contains(&"rb"));
    }

    #[test]
    fn test_load_scripts_nonexistent_directory() {
        let result = load_scripts(PathBuf::from("/nonexistent/path/that/does/not/exist"));

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Failed to read directory"));
    }

    #[test]
    fn test_script_path_preservation() {
        let temp_dir = TempDir::new().unwrap();
        let script_dir = temp_dir.path();

        fs::write(script_dir.join("test.sh"), "#!/bin/bash").unwrap();

        let scripts = load_scripts(script_dir.to_path_buf()).unwrap();

        assert_eq!(scripts.len(), 1);
        assert!(scripts[0].path.ends_with("test.sh"));
        assert!(scripts[0].path.is_absolute());
    }
}
