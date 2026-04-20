use thiserror::Error;

#[derive(Error, Debug)]
pub enum AcrError {
    #[error("Not logged in. Run `acr login` first")]
    NotLoggedIn,

    #[error("Contest '{0}' not found")]
    ContestNotFound(String),

    #[error("Problem '{0}' not found")]
    ProblemNotFound(String),

    #[error("Test failed ({passed}/{total} AC). Use `acr submit --force` to submit anyway")]
    TestFailed { passed: usize, total: usize },

    #[error("Config not found. Run `acr init` first")]
    ConfigNotFound,

    #[error("Contest directory already exists: {0}")]
    ContestAlreadyExists(String),

    #[error("Could not extract CSRF token from submit page")]
    CsrfTokenNotFound,

    #[error("Submission failed: {reason}")]
    SubmissionFailed { reason: String },
}
