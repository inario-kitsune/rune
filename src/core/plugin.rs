use crate::utils::fs::get_plugin_db;
use anyhow::{Context, Result};
use bincode::config;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

/// 插件定义结构体
///
/// 插件描述了如何执行特定类型的脚本文件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugin {
    /// 插件名称（唯一标识）
    pub name: String,
    /// 执行器命令（如 python3, bash, node）
    pub executor: String,
    /// 参数模板，{file} 会被替换为脚本路径
    #[serde(default = "default_arg_template")]
    pub arg_template: Vec<String>,
    /// 支持的文件扩展名列表
    pub extensions: Vec<String>,
    /// 插件描述
    #[serde(default)]
    pub description: String,
    /// 作者信息
    #[serde(default)]
    pub author: String,
    /// 版本号
    #[serde(default)]
    pub version: String,
    /// 依赖的其他命令
    #[serde(default)]
    pub requires: Vec<String>,
}

impl Plugin {
    /// 从 YAML 字符串解析插件
    ///
    /// # 参数
    /// * `yaml` - YAML 格式的插件定义
    ///
    /// # 返回
    /// 解析后的插件对象
    ///
    /// # 错误
    /// - YAML 格式错误
    /// - 缺少必填字段（name, executor, extensions）
    pub fn from_yaml(yaml: &str) -> Result<Self> {
        let plugin: Plugin = serde_yaml::from_str(yaml).context("Failed to parse plugin YAML")?;

        // 验证必填字段
        if plugin.name.is_empty() {
            anyhow::bail!("Plugin name cannot be empty");
        }
        if plugin.executor.is_empty() {
            anyhow::bail!("Plugin executor cannot be empty");
        }
        if plugin.extensions.is_empty() {
            anyhow::bail!("Plugin must support at least one extension");
        }

        Ok(plugin)
    }

    /// 将插件序列化为 YAML 字符串
    pub fn to_yaml(&self) -> Result<String> {
        serde_yaml::to_string(self).context("Failed to serialize plugin to YAML")
    }

    /// 验证插件的执行器和依赖是否可用
    ///
    /// 注意：此方法只会打印警告，不会返回错误
    pub fn validate(&self) -> Result<()> {
        // 检查执行器是否可用
        if which::which(&self.executor).is_err() {
            eprintln!(
                "警告: 执行器 '{}' 未在 PATH 中找到，插件可能无法正常工作。",
                self.executor
            );
        }

        // 检查依赖
        for req in &self.requires {
            if which::which(req).is_err() {
                eprintln!("警告: 依赖命令 '{}' 未在 PATH 中找到。", req);
            }
        }

        Ok(())
    }
}

/// 默认参数模板
fn default_arg_template() -> Vec<String> {
    vec!["{file}".to_string()]
}

/// 插件数据库
///
/// 使用二进制格式存储所有已安装的插件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDatabase {
    /// 插件名称 -> Plugin 映射
    plugins: HashMap<String, Plugin>,
    /// 数据库版本号
    version: u32,
}

/// 当前数据库版本
const DB_VERSION: u32 = 1;

impl PluginDatabase {
    /// 创建新的空数据库
    fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            version: DB_VERSION,
        }
    }

    /// 从磁盘加载插件数据库
    ///
    /// 如果数据库不存在，会自动创建一个空数据库
    pub fn load() -> Result<Self> {
        let db_path = get_plugin_db()?;
        if !db_path.exists() {
            // 数据库不存在，创建默认数据库
            let db = Self::new();
            db.save()?;
            return Ok(db);
        }
        // 读取数据库
        let data = fs::read(&db_path).context("Failed to read plugin database")?;
        // 使用新版 bincode API
        let config = config::standard();
        let (db, _): (Self, usize) = bincode::serde::decode_from_slice(&data, config)
            .context("Failed to deserialize plugin database")?;
        // 验证版本
        if db.version != DB_VERSION {
            anyhow::bail!(
                "插件数据库版本不匹配。期望 {}, 实际 {}",
                DB_VERSION,
                db.version
            );
        }

        Ok(db)
    }

    /// 保存数据库到磁盘
    pub fn save(&self) -> Result<()> {
        let db_path = get_plugin_db()?;
        // 确保目录存在
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let config = config::standard();
        let data = bincode::serde::encode_to_vec(self, config)
            .context("Failed to serialize plugin database")?;
        fs::write(&db_path, data).context("Failed to write plugin database")?;

        Ok(())
    }

    /// 添加新插件
    ///
    /// # 错误
    /// - 插件名称已存在
    /// - 插件验证失败
    pub fn add_plugin(&mut self, plugin: Plugin) -> Result<()> {
        // 验证插件
        plugin.validate()?;

        // 检查是否已存在
        if self.plugins.contains_key(&plugin.name) {
            anyhow::bail!("插件 '{}' 已存在", plugin.name);
        }

        self.plugins.insert(plugin.name.clone(), plugin);
        Ok(())
    }

    /// 更新已存在的插件（覆盖）
    pub fn update_plugin(&mut self, plugin: Plugin) -> Result<()> {
        plugin.validate()?;
        self.plugins.insert(plugin.name.clone(), plugin);
        Ok(())
    }

    /// 移除插件
    ///
    /// # 错误
    /// 插件不存在时返回错误
    pub fn remove_plugin(&mut self, name: &str) -> Result<()> {
        self.plugins
            .remove(name)
            .ok_or_else(|| anyhow::anyhow!("插件 '{}' 不存在", name))?;
        Ok(())
    }

    /// 根据名称获取插件
    pub fn get_plugin(&self, name: &str) -> Option<&Plugin> {
        self.plugins.get(name)
    }

    /// 根据扩展名查找所有匹配的插件
    ///
    /// 返回的插件列表按名称排序
    pub fn find_all_by_extension(&self, extension: &str) -> Vec<&Plugin> {
        let mut plugins: Vec<_> = self
            .plugins
            .values()
            .filter(|p| p.extensions.contains(&extension.to_string()))
            .collect();

        // 按名称排序，保证顺序稳定
        plugins.sort_by(|a, b| a.name.cmp(&b.name));
        plugins
    }

    /// 获取所有插件的迭代器
    pub fn all_plugins(&self) -> impl Iterator<Item = &Plugin> {
        self.plugins.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_plugin_yaml() -> &'static str {
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

    #[test]
    fn test_plugin_from_yaml_valid() {
        let plugin = Plugin::from_yaml(sample_plugin_yaml()).unwrap();

        assert_eq!(plugin.name, "python");
        assert_eq!(plugin.executor, "python3");
        assert_eq!(plugin.arg_template, vec!["{file}"]);
        assert_eq!(plugin.extensions, vec!["py"]);
        assert_eq!(plugin.description, "Python 3 interpreter");
        assert_eq!(plugin.author, "Test Author");
        assert_eq!(plugin.version, "1.0.0");
        assert_eq!(plugin.requires, vec!["python3"]);
    }

    #[test]
    fn test_plugin_from_yaml_minimal() {
        let yaml = r#"
name: bash
executor: bash
extensions:
  - sh
"#;
        let plugin = Plugin::from_yaml(yaml).unwrap();

        assert_eq!(plugin.name, "bash");
        assert_eq!(plugin.executor, "bash");
        assert_eq!(plugin.extensions, vec!["sh"]);
        // Optional fields should have defaults
        assert_eq!(plugin.description, "");
        assert_eq!(plugin.author, "");
        assert_eq!(plugin.version, "");
        assert!(plugin.requires.is_empty());
        assert_eq!(plugin.arg_template, vec!["{file}"]);
    }

    #[test]
    fn test_plugin_from_yaml_missing_name() {
        // YAML with empty name (serde will deserialize as empty string)
        let yaml = r#"
name: ""
executor: python3
extensions:
  - py
"#;
        let result = Plugin::from_yaml(yaml);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Plugin name cannot be empty"));
    }

    #[test]
    fn test_plugin_from_yaml_missing_executor() {
        // YAML with empty executor (serde will deserialize as empty string)
        let yaml = r#"
name: python
executor: ""
extensions:
  - py
"#;
        let result = Plugin::from_yaml(yaml);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Plugin executor cannot be empty"));
    }

    #[test]
    fn test_plugin_from_yaml_missing_extensions() {
        let yaml = r#"
name: python
executor: python3
extensions: []
"#;
        let result = Plugin::from_yaml(yaml);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Plugin must support at least one extension"));
    }

    #[test]
    fn test_plugin_from_yaml_invalid_yaml() {
        let yaml = "invalid: yaml: content: [[[";
        let result = Plugin::from_yaml(yaml);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to parse plugin YAML"));
    }

    #[test]
    fn test_plugin_to_yaml() {
        let plugin = Plugin {
            name: "test".to_string(),
            executor: "test-exec".to_string(),
            arg_template: vec!["{file}".to_string()],
            extensions: vec!["test".to_string()],
            description: "Test plugin".to_string(),
            author: "Author".to_string(),
            version: "1.0".to_string(),
            requires: vec!["dep1".to_string()],
        };

        let yaml = plugin.to_yaml().unwrap();

        assert!(yaml.contains("name: test"));
        assert!(yaml.contains("executor: test-exec"));
        assert!(yaml.contains("test"));
    }

    #[test]
    fn test_plugin_yaml_roundtrip() {
        let original = Plugin::from_yaml(sample_plugin_yaml()).unwrap();
        let yaml = original.to_yaml().unwrap();
        let reconstructed = Plugin::from_yaml(&yaml).unwrap();

        assert_eq!(original.name, reconstructed.name);
        assert_eq!(original.executor, reconstructed.executor);
        assert_eq!(original.extensions, reconstructed.extensions);
        assert_eq!(original.description, reconstructed.description);
    }

    #[test]
    fn test_plugin_validate_always_succeeds() {
        // validate() only prints warnings, doesn't fail
        let plugin = Plugin {
            name: "test".to_string(),
            executor: "nonexistent_command_12345".to_string(),
            arg_template: vec!["{file}".to_string()],
            extensions: vec!["test".to_string()],
            description: "".to_string(),
            author: "".to_string(),
            version: "".to_string(),
            requires: vec!["nonexistent_dep_98765".to_string()],
        };

        // Should succeed even if executor doesn't exist
        assert!(plugin.validate().is_ok());
    }

    #[test]
    fn test_plugin_database_new() {
        let db = PluginDatabase::new();

        assert_eq!(db.version, DB_VERSION);
        assert_eq!(db.all_plugins().count(), 0);
    }

    #[test]
    fn test_plugin_database_add_plugin() {
        let mut db = PluginDatabase::new();
        let plugin = Plugin::from_yaml(sample_plugin_yaml()).unwrap();

        let result = db.add_plugin(plugin.clone());

        assert!(result.is_ok());
        assert_eq!(db.get_plugin("python").unwrap().name, "python");
    }

    #[test]
    fn test_plugin_database_add_duplicate() {
        let mut db = PluginDatabase::new();
        let plugin = Plugin::from_yaml(sample_plugin_yaml()).unwrap();

        db.add_plugin(plugin.clone()).unwrap();
        let result = db.add_plugin(plugin.clone());

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("已存在"));
    }

    #[test]
    fn test_plugin_database_update_plugin() {
        let mut db = PluginDatabase::new();
        let mut plugin = Plugin::from_yaml(sample_plugin_yaml()).unwrap();

        db.add_plugin(plugin.clone()).unwrap();

        // Update the plugin
        plugin.description = "Updated description".to_string();
        db.update_plugin(plugin.clone()).unwrap();

        assert_eq!(db.get_plugin("python").unwrap().description, "Updated description");
    }

    #[test]
    fn test_plugin_database_remove_plugin() {
        let mut db = PluginDatabase::new();
        let plugin = Plugin::from_yaml(sample_plugin_yaml()).unwrap();

        db.add_plugin(plugin).unwrap();
        assert!(db.get_plugin("python").is_some());

        db.remove_plugin("python").unwrap();
        assert!(db.get_plugin("python").is_none());
    }

    #[test]
    fn test_plugin_database_remove_nonexistent() {
        let mut db = PluginDatabase::new();
        let result = db.remove_plugin("nonexistent");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("不存在"));
    }

    #[test]
    fn test_plugin_database_get_plugin() {
        let mut db = PluginDatabase::new();
        let plugin = Plugin::from_yaml(sample_plugin_yaml()).unwrap();

        assert!(db.get_plugin("python").is_none());

        db.add_plugin(plugin).unwrap();

        assert!(db.get_plugin("python").is_some());
        assert_eq!(db.get_plugin("python").unwrap().executor, "python3");
    }

    #[test]
    fn test_plugin_database_find_by_extension_single() {
        let mut db = PluginDatabase::new();
        let plugin = Plugin::from_yaml(sample_plugin_yaml()).unwrap();
        db.add_plugin(plugin).unwrap();

        let matches = db.find_all_by_extension("py");

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].name, "python");
    }

    #[test]
    fn test_plugin_database_find_by_extension_multiple() {
        let mut db = PluginDatabase::new();

        let plugin1 = Plugin {
            name: "python".to_string(),
            executor: "python3".to_string(),
            arg_template: vec!["{file}".to_string()],
            extensions: vec!["py".to_string()],
            description: "".to_string(),
            author: "".to_string(),
            version: "".to_string(),
            requires: vec![],
        };

        let plugin2 = Plugin {
            name: "pypy".to_string(),
            executor: "pypy3".to_string(),
            arg_template: vec!["{file}".to_string()],
            extensions: vec!["py".to_string()],
            description: "".to_string(),
            author: "".to_string(),
            version: "".to_string(),
            requires: vec![],
        };

        db.add_plugin(plugin1).unwrap();
        db.add_plugin(plugin2).unwrap();

        let matches = db.find_all_by_extension("py");

        assert_eq!(matches.len(), 2);
        // Should be sorted by name
        assert_eq!(matches[0].name, "pypy");
        assert_eq!(matches[1].name, "python");
    }

    #[test]
    fn test_plugin_database_find_by_extension_none() {
        let db = PluginDatabase::new();
        let matches = db.find_all_by_extension("xyz");

        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_plugin_database_all_plugins() {
        let mut db = PluginDatabase::new();

        let plugin1 = Plugin::from_yaml(sample_plugin_yaml()).unwrap();
        let plugin2 = Plugin {
            name: "bash".to_string(),
            executor: "bash".to_string(),
            arg_template: vec!["{file}".to_string()],
            extensions: vec!["sh".to_string()],
            description: "".to_string(),
            author: "".to_string(),
            version: "".to_string(),
            requires: vec![],
        };

        db.add_plugin(plugin1).unwrap();
        db.add_plugin(plugin2).unwrap();

        assert_eq!(db.all_plugins().count(), 2);
    }
}
