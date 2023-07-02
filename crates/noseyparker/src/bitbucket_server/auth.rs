use secrecy::SecretString;

// -------------------------------------------------------------------------------------------------
// Auth
// -------------------------------------------------------------------------------------------------
/// Supported forms of authentication
pub enum Auth {
    Unauthenticated,
    /// Authenticate with a Bitbucket Server Personal Access Token
    PersonalAccessToken(SecretString),
}
