use std::process::{Command, Stdio};

use anyhow::{Context, Result};

/// 命令执行器
///
/// 用于执行外部命令，支持参数传递和错误处理
pub struct CommandExecutor {
    /// 要执行的命令
    command: String,
    /// 命令参数列表
    args: Vec<String>,
}

impl CommandExecutor {
    /// 创建新的命令执行器
    ///
    /// # 参数
    /// * `command` - 要执行的命令（如 "python3", "bash"）
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            args: Vec::new(),
        }
    }

    /// 添加单个参数
    ///
    /// 支持链式调用
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// 添加多个参数
    ///
    /// 支持链式调用
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.args.extend(args.into_iter().map(|s| s.into()));
        self
    }

    /// 检查命令是否在 PATH 中可用
    ///
    /// # 错误
    /// 如果命令不存在，返回错误
    pub fn check_available(&self) -> Result<()> {
        which::which(&self.command)
            .with_context(|| format!("命令 '{}' 未在 PATH 中找到", self.command))?;
        Ok(())
    }

    /// 执行命令
    ///
    /// 命令将继承当前进程的 stdin/stdout/stderr，支持交互式程序
    ///
    /// # 错误
    /// - 命令不存在
    /// - 命令执行失败
    /// - 命令返回非零退出码
    pub fn execute(&self) -> Result<()> {
        self.check_available()?;

        let status = Command::new(&self.command)
            .args(&self.args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .with_context(|| format!("执行命令失败: {}", self.command))?;

        if !status.success() {
            anyhow::bail!(
                "命令 '{}' 退出码: {}",
                self.command,
                status.code().unwrap_or(-1)
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_executor_new() {
        let executor = CommandExecutor::new("echo");
        assert_eq!(executor.command, "echo");
        assert_eq!(executor.args.len(), 0);
    }

    #[test]
    fn test_command_executor_arg() {
        let executor = CommandExecutor::new("echo").arg("hello");
        assert_eq!(executor.args, vec!["hello"]);
    }

    #[test]
    fn test_command_executor_args_chaining() {
        let executor = CommandExecutor::new("echo")
            .arg("hello")
            .arg("world")
            .arg("test");
        assert_eq!(executor.args, vec!["hello", "world", "test"]);
    }

    #[test]
    fn test_command_executor_args_vec() {
        let executor = CommandExecutor::new("echo")
            .args(vec!["hello", "world", "test"]);
        assert_eq!(executor.args, vec!["hello", "world", "test"]);
    }

    #[test]
    fn test_command_executor_check_available_valid() {
        // Test with a command that should exist on all systems
        let executor = CommandExecutor::new("echo");
        assert!(executor.check_available().is_ok());
    }

    #[test]
    fn test_command_executor_check_available_invalid() {
        let executor = CommandExecutor::new("nonexistent_command_xyz_12345");
        let result = executor.check_available();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("未在 PATH 中找到"));
    }

    #[test]
    fn test_command_executor_execute_simple() {
        // Execute a simple echo command
        let result = CommandExecutor::new("echo")
            .arg("test")
            .execute();

        assert!(result.is_ok());
    }

    #[test]
    fn test_command_executor_execute_nonexistent() {
        let result = CommandExecutor::new("nonexistent_command_xyz_12345")
            .execute();

        assert!(result.is_err());
    }

    #[test]
    fn test_command_executor_execute_failing_command() {
        // 使用一个会失败的命令（退出码非零）
        // false 命令在 Unix 系统上存在且总是返回 1
        #[cfg(not(target_os = "windows"))]
        {
            let result = CommandExecutor::new("false").execute();
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("退出码"));
        }

        // Windows 上使用一个失败的 PowerShell 命令
        #[cfg(target_os = "windows")]
        {
            let result = CommandExecutor::new("cmd")
                .arg("/C")
                .arg("exit 1")
                .execute();
            assert!(result.is_err());
        }
    }
}
