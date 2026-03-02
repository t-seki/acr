use thiserror::Error;

#[derive(Error, Debug)]
pub enum AcrsError {
    #[error("Not logged in. Run `acrs login` first")]
    NotLoggedIn,

    #[error("Contest '{0}' not found")]
    ContestNotFound(String),

    #[error("Problem '{0}' not found")]
    ProblemNotFound(String),

    #[error("Test failed ({passed}/{total} AC)")]
    TestFailed { passed: usize, total: usize },

    #[error("Scraping failed: {0}")]
    ScrapingFailed(String),

    #[error("Config not found. Run `acrs init` first")]
    ConfigNotFound,

    #[error("Contest directory already exists: {0}")]
    ContestAlreadyExists(String),
}
