use std::fmt;

#[derive(Debug)]
pub enum GitCatError {
    CommandFailed { command: String, stderr: String },
    NotARepository,
    GitError(String),
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    MissingArgument(String),
}

impl fmt::Display for GitCatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitCatError::CommandFailed { command, stderr } => {
                write!(f, "Command '{}' failed: {}", command, stderr)
            }
            GitCatError::NotARepository => {
                write!(f, "Not a git repository")
            }
            GitCatError::GitError(msg) => write!(f, "Git error: {}", msg),
            GitCatError::Io(err) => write!(f, "IO error: {}", err),
            GitCatError::Utf8(err) => write!(f, "UTF-8 conversion error: {}", err),
            GitCatError::MissingArgument(arg) => {
                write!(f, "Missing required argument: {}", arg)
            }
        }
    }
}

impl std::error::Error for GitCatError {}

impl From<std::io::Error> for GitCatError {
    fn from(err: std::io::Error) -> Self {
        GitCatError::Io(err)
    }
}

impl From<std::string::FromUtf8Error> for GitCatError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        GitCatError::Utf8(err)
    }
}

/// Type alias for Result with GitCatError
pub type Result<T> = std::result::Result<T, GitCatError>;
