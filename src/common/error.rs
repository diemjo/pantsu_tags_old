use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // sauce errors
    #[error("Failed to send image source request: {0}")]
    FailedRequest(#[from] reqwest::Error),

    #[error("Received response with bad http status: {0}")]
    BadResponseStatus(StatusCode),

    #[error("Failed to find sauces for image: {0}")]
    NoSaucesFound(String),

    #[error("Failed to find tags on image website: {0}")]
    NoTagsFound(String),

    #[error("Error getting tags from html.")]
    ErrorGettingTags,

    // pantsu tag database errors
    #[error("Failed to add tag '{2}' for file '{1}': {0}")]
    TagInsertionError(#[source] rusqlite::Error, String /* file */, String /* tag */),

    #[error("Failed to remove tag '{2}' from file '{1}': {0}")]
    TagRemovalError(#[source] rusqlite::Error, String /* file */, String /* tag */),

    #[error("Failed underlying SQLite call: {0}")]
    SQLError(#[from] rusqlite::Error),

    #[error("Cannot convert invalid tag type '{0}' to enum variant of PantsuTagType")]
    InvalidTagType(String),

    #[error("Cannot convert tag string '{0}' to PantsuTag")]
    InvalidTagFormat(String),

    // file system
    #[error("File not found: {1}")]
    FileNotFound(#[source] std::io::Error, String),

    #[error("Error creating dir {1}: {0}")]
    DirectoryCreateError(#[source] std::io::Error, String)
}