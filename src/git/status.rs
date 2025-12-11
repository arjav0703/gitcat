#[derive(Debug)]
pub enum Status {
    Clean,
    Unstaged,
    Staged,
}

impl Status {
    pub fn from_str(s: &str) -> Self {
        if s.contains("nothing to commit, working tree clean") {
            Status::Clean
        } else if s.contains("Changes not staged for commit") {
            Status::Unstaged
        } else if s.contains("Changes to be committed") {
            Status::Staged
        } else {
            Status::Clean
        }
    }
}
