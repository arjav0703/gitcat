use crate::command::CommandExecutor;
use crate::config::Config;
use crate::error::{GitCatError, Result};
use std::path::Path;

mod status;
pub use status::Status;

pub struct GitRepository {
    config: Config,
}

impl GitRepository {
    pub fn new() -> Self {
        Self {
            config: Config::new(),
        }
    }

    pub fn is_repository() -> Result<bool> {
        Ok(Path::new(".git").exists())
    }

    pub async fn status(&self) -> Result<Status> {
        let output = CommandExecutor::execute(&["status"])?;
        let stdout = CommandExecutor::stdout_string(&output)?;
        Ok(Status::from_output(&stdout))
    }

    pub async fn display_status(&self) -> Result<()> {
        let status = self.status().await?;
        let message = match status {
            Status::Clean => self.config.status_clean_msg(),
            Status::Unstaged => self.config.status_unstaged_msg(),
            Status::Staged => self.config.status_staged_msg(),
            Status::Mixed => self.config.status_staged_msg(), // Both staged and unstaged
            Status::Conflict => "⚠️  Merge conflicts detected!",
        };
        println!("{}", message);
        Ok(())
    }

    pub async fn add(&self, args: &[String]) -> Result<()> {
        let output = CommandExecutor::execute_with_args(&["add"], args)?;

        if CommandExecutor::is_success(&output) {
            println!("{}", self.config.add_success_msg());
        } else {
            let stderr = CommandExecutor::stderr_string(&output)?;
            return Err(GitCatError::CommandFailed {
                command: "add".to_string(),
                stderr,
            });
        }
        Ok(())
    }

    pub async fn commit(&self, args: &[String]) -> Result<()> {
        let output = CommandExecutor::execute_with_args(&["commit"], args)?;

        if CommandExecutor::is_success(&output) {
            let stdout = CommandExecutor::stdout_string(&output)?;
            println!("{}", self.config.commit_success_msg());
            println!("{}", stdout);
        } else {
            let stdout = CommandExecutor::stdout_string(&output)?;
            if stdout.contains("nothing to commit") {
                println!("{}", self.config.commit_nothing_msg());
            } else {
                let stderr = CommandExecutor::stderr_string(&output)?;
                return Err(GitCatError::CommandFailed {
                    command: "commit".to_string(),
                    stderr,
                });
            }
        }
        Ok(())
    }

    pub async fn push(&self, args: &[String]) -> Result<()> {
        let output = CommandExecutor::execute_with_args(&["push"], args)?;

        if CommandExecutor::is_success(&output) {
            let stdout = CommandExecutor::stdout_string(&output)?;
            if stdout.contains("Everything up-to-date") {
                println!("{}", self.config.push_uptodate_msg());
            } else {
                println!("{}", self.config.push_success_msg());
            }
        } else {
            let stderr = CommandExecutor::stderr_string(&output)?;
            return Err(GitCatError::CommandFailed {
                command: "push".to_string(),
                stderr,
            });
        }
        Ok(())
    }

    pub async fn pull(&self, args: &[String]) -> Result<()> {
        let output = CommandExecutor::execute_with_args(&["pull"], args)?;

        if CommandExecutor::is_success(&output) {
            println!("{}", self.config.pull_success_msg());
        } else {
            let stderr = CommandExecutor::stderr_string(&output)?;
            return Err(GitCatError::CommandFailed {
                command: "pull".to_string(),
                stderr,
            });
        }
        Ok(())
    }

    pub async fn checkout(&self, branch: &str) -> Result<()> {
        let output = CommandExecutor::execute(&["checkout", branch])?;

        if CommandExecutor::is_success(&output) {
            println!("{}", self.config.checkout_success_msg(branch));
        } else {
            let stderr = CommandExecutor::stderr_string(&output)?;
            return Err(GitCatError::CommandFailed {
                command: format!("checkout {}", branch),
                stderr,
            });
        }
        Ok(())
    }

    pub async fn create_branch(&self, branch: &str) -> Result<()> {
        let output = CommandExecutor::execute(&["checkout", "-b", branch])?;

        if CommandExecutor::is_success(&output) {
            println!("{}", self.config.branch_create_success_msg(branch));
        } else {
            let stderr = CommandExecutor::stderr_string(&output)?;
            return Err(GitCatError::CommandFailed {
                command: format!("checkout -b {}", branch),
                stderr,
            });
        }
        Ok(())
    }

    pub async fn diff(&self, args: &[String]) -> Result<()> {
        let mut base_args = vec!["diff"];
        let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        base_args.extend(args_refs);

        CommandExecutor::execute_interactive(&base_args)?;
        Ok(())
    }

    pub async fn stash(&self) -> Result<()> {
        let output = CommandExecutor::execute(&["stash"])?;

        if CommandExecutor::is_success(&output) {
            println!("{}", self.config.stash_success_msg());
        } else {
            let stderr = CommandExecutor::stderr_string(&output)?;
            return Err(GitCatError::CommandFailed {
                command: "stash".to_string(),
                stderr,
            });
        }
        Ok(())
    }

    pub async fn unstash(&self) -> Result<()> {
        let output = CommandExecutor::execute(&["stash", "pop"])?;

        if CommandExecutor::is_success(&output) {
            println!("{}", self.config.unstash_success_msg());
        } else {
            let stderr = CommandExecutor::stderr_string(&output)?;
            return Err(GitCatError::CommandFailed {
                command: "stash pop".to_string(),
                stderr,
            });
        }
        Ok(())
    }

    pub async fn show_stash_list(&self) -> Result<()> {
        CommandExecutor::execute_interactive(&["stash", "list"])?;
        Ok(())
    }

    async fn save_config(&self) -> anyhow::Result<()> {
        use dirs::config_dir;

        let config_path = config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("gitcat")
            .join("config");

        tokio::fs::create_dir_all(config_path.parent().unwrap()).await?;
        let data = self.config.catmood.to_string();
        // dbg!(&data, &config_path);
        tokio::fs::write(config_path, data).await?;
        Ok(())
    }

    pub async fn set_mood(&mut self, mood: &str) -> anyhow::Result<()> {
        match mood {
            "chaotic" => self.config.catmood = crate::config::CatMood::Chaotic,
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid mood '{}'. Supported moods: chaotic",
                    mood
                ));
            }
        }
        self.save_config().await?;
        Ok(())
    }
}

impl Default for GitRepository {
    fn default() -> Self {
        Self::new()
    }
}
