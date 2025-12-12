use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Clean,
    Unstaged,
    Staged,
    Mixed,
    Conflict,
}

impl Status {
    pub fn from_output(output: &str) -> Self {
        let has_conflicts = output.contains("both modified:")
            || output.contains("both added:")
            || output.contains("Unmerged paths:");

        if has_conflicts {
            return Status::Conflict;
        }

        let has_staged = output.contains("Changes to be committed");
        let has_unstaged =
            output.contains("Changes not staged for commit") || output.contains("Untracked files");

        match (has_staged, has_unstaged) {
            (true, true) => Status::Mixed,
            (true, false) => Status::Staged,
            (false, true) => Status::Unstaged,
            (false, false) => Status::Clean,
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Clean => write!(f, "clean"),
            Status::Unstaged => write!(f, "unstaged changes"),
            Status::Staged => write!(f, "staged changes"),
            Status::Mixed => write!(f, "mixed changes"),
            Status::Conflict => write!(f, "conflicts"),
        }
    }
}
