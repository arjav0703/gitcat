/// Configuration for gitcat messages and behavior
pub struct Config {
    pub cat_themed: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { cat_themed: true }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status_clean_msg(&self) -> &str {
        if self.cat_themed {
            "😺 Purrfect! As clean as my furr."
        } else {
            "✓ Working tree is clean"
        }
    }

    pub fn status_unstaged_msg(&self) -> &str {
        if self.cat_themed {
            "🐾 You've been scratching things again!"
        } else {
            "! Unstaged changes detected"
        }
    }

    pub fn status_staged_msg(&self) -> &str {
        if self.cat_themed {
            "😸 Ready to pounce on that commit!"
        } else {
            "✓ Changes staged for commit"
        }
    }

    pub fn commit_success_msg(&self) -> &str {
        if self.cat_themed {
            "😺 Your changes are ready to be sent to meowland!"
        } else {
            "✓ Changes committed successfully"
        }
    }

    pub fn commit_nothing_msg(&self) -> &str {
        if self.cat_themed {
            "😺 Nothing to commit! Your code is already purrfect!"
        } else {
            "! Nothing to commit, working tree clean"
        }
    }

    pub fn push_success_msg(&self) -> &str {
        if self.cat_themed {
            "🚀 Pushed your beautiful code to meowland!"
        } else {
            "✓ Pushed successfully"
        }
    }

    pub fn push_uptodate_msg(&self) -> &str {
        if self.cat_themed {
            "😺 Everything is already up-to-date in meowland!"
        } else {
            "✓ Everything up-to-date"
        }
    }

    pub fn pull_success_msg(&self) -> &str {
        if self.cat_themed {
            "⬇️ Fetched the latest meow updates from meowland!"
        } else {
            "✓ Pulled updates successfully"
        }
    }

    pub fn checkout_success_msg(&self, branch: &str) -> String {
        if self.cat_themed {
            format!("😺 Pounced to branch '{}'", branch)
        } else {
            format!("✓ Switched to branch '{}'", branch)
        }
    }

    pub fn branch_create_success_msg(&self, branch: &str) -> String {
        if self.cat_themed {
            format!("😺 Created and pounced to new branch '{}'", branch)
        } else {
            format!("✓ Created and switched to branch '{}'", branch)
        }
    }

    pub fn stash_success_msg(&self) -> &str {
        if self.cat_themed {
            "Your changes will be safe in my dreams 😴💤!"
        } else {
            "✓ Changes stashed successfully"
        }
    }

    pub fn unstash_success_msg(&self) -> &str {
        if self.cat_themed {
            "Welcome back to reality! Your changes are restored 😺!"
        } else {
            "✓ Stashed changes restored"
        }
    }

    pub fn error_prefix(&self) -> &str {
        if self.cat_themed { "🐾" } else { "✗" }
    }
}
