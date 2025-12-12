#[derive(Debug, Clone)]
pub struct Config {
    pub catmood: CatMood,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            catmood: CatMood::Chaotic,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatMood {
    Chaotic,
}

impl ToString for CatMood {
    fn to_string(&self) -> String {
        match self {
            CatMood::Chaotic => "chaotic".to_string(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status_clean_msg(&self) -> &str {
        match self.catmood {
            CatMood::Chaotic => "😺 Purrfect! As clean as my furr.",
        }
    }

    pub fn status_unstaged_msg(&self) -> &str {
        match self.catmood {
            CatMood::Chaotic => "🐾 You've been scratching things again!",
        }
    }

    pub fn status_staged_msg(&self) -> &str {
        match self.catmood {
            CatMood::Chaotic => "😸 Ready to pounce on that commit!",
        }
    }

    pub fn commit_success_msg(&self) -> &str {
        match self.catmood {
            CatMood::Chaotic => "😺 Your changes are ready to be sent to meowland!",
        }
    }

    pub fn commit_nothing_msg(&self) -> &str {
        match self.catmood {
            CatMood::Chaotic => "😺 Nothing to commit! Your code is already purrfect!",
        }
    }

    pub fn push_success_msg(&self) -> &str {
        match self.catmood {
            CatMood::Chaotic => "🚀 Pushed your beautiful code to meowland!",
        }
    }

    pub fn push_uptodate_msg(&self) -> &str {
        match self.catmood {
            CatMood::Chaotic => "😺 Everything is already up-to-date in meowland!",
        }
    }

    pub fn pull_success_msg(&self) -> &str {
        match self.catmood {
            CatMood::Chaotic => "⬇️ Fetched the latest meow updates from meowland!",
        }
    }

    pub fn checkout_success_msg(&self, branch: &str) -> String {
        match self.catmood {
            CatMood::Chaotic => format!("😺 Pounced to branch '{}'", branch),
        }
    }

    pub fn branch_create_success_msg(&self, branch: &str) -> String {
        match self.catmood {
            CatMood::Chaotic => format!("😺 Created and pounced to new branch '{}'", branch),
        }
    }

    pub fn stash_success_msg(&self) -> &str {
        match self.catmood {
            CatMood::Chaotic => "Your changes will be safe in my dreams UwU💤 !",
        }
    }

    pub fn unstash_success_msg(&self) -> &str {
        match self.catmood {
            CatMood::Chaotic => "Welcome back to reality! Your changes are restored OwO",
        }
    }

    pub fn add_success_msg(&self) -> &str {
        match self.catmood {
            CatMood::Chaotic => "😸 Files added to the staging area! Ready to pounce >//<",
        }
    }

    pub fn error_prefix(&self) -> &str {
        match self.catmood {
            CatMood::Chaotic => "😿 whoops! Something went wrong: ",
        }
    }
}
