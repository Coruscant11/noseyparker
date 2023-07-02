use super::error::Error;

// -------------------------------------------------------------------------------------------------
// Result
// -------------------------------------------------------------------------------------------------
pub type Result<T> = std::result::Result<T, Error>;