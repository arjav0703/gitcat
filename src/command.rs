use crate::error::{GitCatError, Result};
use std::process::{Command, Output, Stdio};

/// Executes a git command and returns the output
pub struct CommandExecutor;

impl CommandExecutor {
    pub fn execute(args: &[&str]) -> Result<Output> {
        let output = Command::new("git")
            .args(args)
            .output()
            .map_err(GitCatError::Io)?;

        Ok(output)
    }

    pub fn execute_with_args(base_args: &[&str], additional_args: &[String]) -> Result<Output> {
        let output = Command::new("git")
            .args(base_args)
            .args(additional_args)
            .output()
            .map_err(GitCatError::Io)?;

        Ok(output)
    }

    /// inherit stdout/stderr for live output
    pub fn execute_interactive(args: &[&str]) -> Result<()> {
        let status = Command::new("git")
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .map_err(GitCatError::Io)?;

        if !status.success() {
            return Err(GitCatError::GitError(format!(
                "Command 'git {}' failed with exit code: {}",
                args.join(" "),
                status.code().unwrap_or(-1)
            )));
        }

        Ok(())
    }

    pub fn is_success(output: &Output) -> bool {
        output.status.success()
    }

    pub fn stdout_string(output: &Output) -> Result<String> {
        String::from_utf8(output.stdout.clone()).map_err(GitCatError::Utf8)
    }

    pub fn stderr_string(output: &Output) -> Result<String> {
        String::from_utf8(output.stderr.clone()).map_err(GitCatError::Utf8)
    }
}
